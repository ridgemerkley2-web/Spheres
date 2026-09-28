# Bharatiya Janata Party presidents 27: the party office, 1990-2026

Packet: **CLAUDE-C01-27**. State: **ready_for_review** (not complete).
Owner: Claude. Integrator/reviewer: Codex. Branch `claude/c01-in-27`; claim commit `27c52dc4`, made while the packet was
stacked on CLAUDE-C01-20 (`claude/c01-in-20` at `2ee3b146`). CLAUDE-C01-20 has since been integrated into
`codex/campaign-certification`, and this branch has merged current integration three times: `da49f9c0` (integration at
`a33a8987`), `26a07bb8` (at `01d5217c`) and `29f1937e` (at `e41aa18d`), so it is based on `codex/campaign-certification`
at `e41aa18d`. Research access: 26 to 28 September 2026 (UTC); every recorded response was downloaded again on 28
September 2026, which is each source's `accessed_date`. The historical cutoff stays **7 September 2026**.

This packet adds one party role to [india.json](india.json): `in_bjp_president` ("National President of the Bharatiya
Janata Party", kind `party_leader`) on the existing recognition observation `in_eci_20240323_np_03` (Bharatiya Janata
Party). The observation's identity, recognition row, lifecycle (`unresearched`), coverage status
(`reporting_identity_only`) and empty game mapping are unchanged; only its sources, claim ids, role list and one coverage
note grow. The packet adds 74 sources and 167 claims, appended after the CLAUDE-C01-20 sources, nineteen holder
observations (three with a stated start and one with a stated end), a role scope note and one bounded note in the packet
coverage. It adds no organization, institution, game mapping, lifespan, portrait or avatar, and it changes nothing that
CLAUDE-C01-11, CLAUDE-C01-15 or CLAUDE-C01-20 added: the thirteen prime-minister, eight president and ten
Congress-president holders are unchanged, and no existing extract is edited. The party office and the
`in_prime_minister` and `in_presidency` institutions stay separate both ways, and `in_inc_president` does not change. The
parent scope (C01, C06, S23, WC1 and CP1) remains open.

The research was done in three parts (A: 1990-1998; B: 1998-2006; C: 2009-2026), and each part was checked independently
before this packet was written. Every checker defect is applied or its outcome explained (see
[Checker defects](#checker-defects)), and every missing primary record the checks confirmed is imported or declined with a
reason.

## Outcome

| ID | Question | Decision |
|---|---|---|
| BJP-PRES-01 | The President when the period opens (L. K. Advani) and the 1991 change | **Accepted in part:** observed on 8 Jan 1991 (Rajya Sabha written answer, in English, official debates store; the date line is printed on the continuation of the same sheet, recorded as a date anchor); the party's National Executive resolutions of April, July and November 1990 style him President under multi-day headings (the April one narrates 10 Dec 1989); the farewell announcing Dr. Joshi's taking over 'tomorrow' is printed twice under conflicting headings (31 Jan and 1-3 Feb 1991) and names no speaker; the address of 1 Feb 1991 says he declined a third term; no record attests 1 Jan 1990 itself or states the day the office changed hands |
| BJP-PRES-02 | Murli Manohar Joshi's presidency | **Accepted in part:** observed on 10 Dec 1991 (Rajya Sabha, item headed 'REGARDING "EKTA YATRA" BY B.J.P. PRESIDENT'); two written answers of 4 Mar 1992 (one in Hindi) and the party's resolution of 27 Feb 1993 attest him again; the resolutions of March and December 1992 style him President under multi-day headings; 'Ex-President' by the National Executive of 18-19 Dec 1993; no record of his election or of the day he assumed or left the office |
| BJP-PRES-03 | Advani's return (1993) | **Accepted in part:** the unnamed addresses of 18 Jun 1993 ('elected me once again President') and 10 Nov 1995 ('for yet another term') are claims; the resolutions of December 1993, February 1996 and April 1998 style him President under multi-day headings; observed on 16 Jul 1997 by the party's 'Press Release by President, Shri Lal Krishna Advani'; the handover was announced on 2 May 1998 for the next day; the bio-data's 'July, 1993' and 'May 2, 1998' are retrospective |
| BJP-PRES-04 | Kushabhau Thakre (1998) and Bangaru Laxman (2000) | **Accepted in part:** Thakre elected unanimously and styled President-elect on 2 May 1998 (no day of election printed); his 'I assume this office' at the session of 3 and 4 May 1998 has no printed day; observed on 27 Feb 1999; Laxman's nominations filed on 2 Aug 2000, his address to the National Council of 27-28 Aug 2000, observed on 31 Aug 2000; his resignation accepted 'with immediate effect' by the office bearers on 14 Mar 2001, the one stated end |
| BJP-PRES-05 | K. Jana Krishnamurthi (2001) and M. Venkaiah Naidu (2002) | **Accepted in part:** Jana Krishnamurthi designated acting president on 14 Mar 2001 (claims only); styled 'National President' on 24 Mar 2001, when he says the Executive has entrusted the presidentship to him, and still on 24 Jun 2002, but no source states that the acting service became a presidency (claims only); Naidu took over from him on 1 Jul 2002 by the party journal (stated start); no record of the body that chose Naidu, and no stated end for either |
| BJP-PRES-06 | Advani (2004) and Rajnath Singh (2005) | **Accepted in part:** on 18 Oct 2004 Naidu's resignation accepted without a stated day of effect and the office bearers resolved to appoint Advani, ratification to follow; Advani observed on 20 Oct 2004 (his 'two days ago' is a recollection); the National Council endorsed his election on 27 Oct 2004 (the release of 30 Oct names the National Executive); his resignation rejected on 8 Jun 2005; 'the last meeting which I shall be presiding over' on 26 Dec 2005; Rajnath Singh observed on 2 Jan 2006; no record of Rajnath Singh's election or of the day either took charge or left the office |
| BJP-PRES-07 | Nitin Gadkari (2009) and Rajnath Singh (2013) | **Accepted in part:** the profile headed 'BJP National President' carries two item dates (18 and 19 Dec 2009) and dates no holder; Gadkari observed on 24 Dec 2009 and on 22 Jan 2013, the day he decided not to seek a second term; 'former BJP National President' by 27 Jan 2013; Rajnath Singh declared elected unopposed on 23 Jan 2013 and styled 'Newly Elected President' that day; observed on 2 Mar 2013; no record of the day either took charge or left the office |
| BJP-PRES-08 | Amit Shah (2014, re-elected 2016) | **Accepted in part:** observed on 9 Jul 2014 (the day the Parliamentary Board thanked the 'outgoing' Rajnath Singh), on 2 Feb 2016 (after the re-election reported on 24 Jan 2016) and on 17 Jun 2019 (the Parliamentary Board meeting that made J. P. Nadda Working President); his request to be relieved and his continuation until the organisational elections (17 Jun 2019) and his handover of charge (20 Jan 2020) are claims; no record of the day he took charge in 2014 or left the office |
| BJP-PRES-09 | J. P. Nadda (Working President 2019, President 2020, extensions) | **Accepted in part:** National Working President from 17 Jun 2019 (claims only); elected unopposed and took charge on 20 Jan 2020 (stated start, the party organ's report); observed on 17 Jan 2023, the day the National Executive extended his tenure 'till June 2024', and on 15 Dec 2025; no further extension found; his handover of charge on 20 Jan 2026 is never an until |
| BJP-PRES-10 | The President at the cutoff, and a party attestation before it | **Accepted:** Nitin Nabin, National Working President from 14-15 Dec 2025 (claims only), the only name proposed on 19 Jan 2026, declared elected and assumed charge on 20 Jan 2026 (stated start, from the party's same-day release and the party organ); observed on 17 Aug 2026 by the party organ's page captured on 4 Sep 2026 |

The resulting holder observations of `in_bjp_president`, in date order:

| Holder | `attested_on` | `from` | `until` | Basis |
|---|---|---|---|---|
| L. K. Advani | 1991-01-08 | null | null | Rajya Sabha written answers, 8 Jan 1991: the Minister of State for Home Affairs, 'Shri L. K. Advani, President BJP' (in English, official debates store; dated by the same sheet's next column) |
| Murli Manohar Joshi | 1991-12-10 | null | null | Rajya Sabha, 10 Dec 1991: Shri V. Narayanasamy, 'this Ekta Yatra, by the BJP President, Dr. Murli Manohar Joshi' (in English, official debates store) |
| L. K. Advani | 1997-07-16 | null | null | the party's 'Press Release by President, Shri Lal Krishna Advani', 16 Jul 1997: 'As President of the BJP' |
| Kushabhau Thakre | 1999-02-27 | null | null | the party's 'Statement on Union Budget 1999-2000 issued by Shri Kushabhau Thakre, President, BJP' |
| Bangaru Laxman | 2000-08-31 | null | 2001-03-14 | the party's 'Statement issued by Shri Bangaru Laxman, President BJP'; end: the office bearers' release of 14 Mar 2001, 'unanimously decided to accept the resignation with immediate effect' |
| M. Venkaiah Naidu | null | 2002-07-01 | null | BJP Today, July 16-31, 2002: 'he took over from Shri Jana Krishnamurthi on July 1, his 53rd birth anniversary as the president' |
| L. K. Advani | 2004-10-20 | null | null | the party's 'Statement issued by Shri L.K. Advani President' at his press conference |
| Rajnath Singh | 2006-01-02 | null | null | 'The statement released by the BJP National President, Shri Rajnath Singh' |
| Nitin Gadkari | 2009-12-24 | null | null | the party's statement 'in my first press conference as President of the BJP' |
| Nitin Gadkari | 2013-01-22 | null | null | 'Press statement issued by BJP National President, Shri Nitin Gadkari' |
| Rajnath Singh | 2013-03-02 | null | null | the party's page 'Presidential speech by Shri Rajnath Singh at BJP National Council Meeting at Talkatora Stadium', dated 2 Mar 2013 |
| Amit Shah | 2014-07-09 | null | null | 'Press : Profile of BJP National President, Shri Amit Shah' |
| Amit Shah | 2016-02-02 | null | null | 'Press : Members from different communities felicitating BJP National President, Shri Amit Shah at 11, Ashok Road' |
| Amit Shah | 2019-06-17 | null | null | Kamal Sandesh: 'Union Home Minister and BJP National President Shri Amit Shah' at the Parliamentary Board meeting |
| J. P. Nadda | null | 2020-01-20 | null | Kamal Sandesh, Vol. 15, No. 03: he 'took charge from the outgoing Party National President and Union Home Minister Shri Amit Shah on 20 January, 2020' |
| J. P. Nadda | 2023-01-17 | null | null | 'Letter : Hon'ble BJP National President Shri J.P. Nadda to the Party Karyakartas' |
| J. P. Nadda | 2025-12-15 | null | null | Kamal Sandesh, Vol. 20, No. 24: 'BJP National President and Union Minister Shri JP Nadda' |
| Nitin Nabin | null | 2026-01-20 | null | the party's release of 20 Jan 2026 ('has assumed charge as the National President') and Kamal Sandesh, Vol. 21, No. 02 ('formally assumed charge ... on 20 January 2026') |
| Nitin Nabin | 2026-08-17 | null | null | Kamal Sandesh's page (capture of 4 Sep 2026): 'Hon'ble National President Shri Nitin Nabin is pleased to appoint' the office bearers 'on 17 August 2026' |

### How a start and an end are decided

One rule covers all three parts, taken from the stacked India packets and, for a party office, the Indian National
Congress presidents packet (CLAUDE-C01-20) and the ANC presidents packet (CLAUDE-C01-16). A holder has `from` only where a
source states the day the office was assumed or taken over, and `until` only where a source states the day it ended;
otherwise the holder is dated by `attested_on`, from a same-day in-office attestation that names the holder and styles
the office. Each observation cites only in-office attestations of its own day, and a start or an end cites only the
claims that state it. Three holders have a stated start (M. Venkaiah Naidu from the party journal, J. P. Nadda from the
party organ, Nitin Nabin from a same-day party release and the party organ) and one has a stated end (Bangaru Laxman,
whose resignation the office bearers accepted 'with immediate effect'). The observations stay separate (the ANC and INC
model), and a holder with a start is followed by later observations of the same person, as in the PT presidents packet
(CLAUDE-C01-22).

Kept as claims that never feed a holder: resolutions and addresses printed under multi-day meeting headings, the party's
election steps (schedules, nominations, scrutiny, sole-candidate announcements, results, declarations and certificates),
President-elect and 'Newly Elected' stylings, acceptances, National Council and National Executive endorsements and
entrustments, an appointment awaiting ratification, prospective announcements, an undated assumption, handover
statements, farewells, 'outgoing', 'former' and 'Ex' stylings, predecessor references, resignations and their
consideration, acceptance without a stated day of effect, rejection or recording, a decision not to seek another term,
the acting presidency of March 2001, the working presidencies of 2019 and 2025, a term extension, later attestations of a
holder already observed (`in_office_continuation_attestation`), retrospective spans, lists, biographies, a profile
heading whose page and the site's listing print different days (Gadkari, December 2009), recollections of an assumption
or a selection, and addresses that name no speaker. A dated release whose own heading names and styles the holder, such
as the profile released on 9 July 2014, is a same-day in-office attestation. Parliament records are used only where they
record the party office: Rajya Sabha records date Advani and Joshi in 1991, because the party's own records of 1990-1992
print multi-day meeting headings or name no speaker.

Rulings on the questions the parts and checks left open:

- **Section headings never supply a speaker.** The 2016 compilation Vol-3 prints the outgoing President's 1991 farewell
  twice, under the section heading 'DR. MM JOSHI' at p.28 and under 'SHRI LK ADVANI' at p.85, so the compiler's headings
  contradict each other. Every address in Vol-2 and Vol-3 whose own text names no speaker has holder_name null and never
  dates a holder, and the Calcutta address of 10 April 1993, which says nothing about the office, is dropped (checks A1,
  A2 and A10).
- **Multi-day headings give no day.** A resolution printed under a heading such as 'National Executive / Madras 21-23
  July, 1990' stores no structured date, whatever it says of the President; single-day headings keep their day (check
  A3).
- **Advani is observed on 8 January 1991, and that page is dated by the rest of its sheet.** The Rajya Sabha page of
  Unstarred Question 1586 prints no date line; the store file of Question 1587 holds the same printed sheet and its next
  column, which prints '[ 8 JAN. 1991 ]'. That file is recorded in the extract as a date anchor with its own byte count
  and SHA-256 (check A14).
- **The 1991 change has no day.** The farewell announcing that Dr. Joshi 'would be taking over as President' the next day
  is printed under 'National Council / New Delhi 31 January, 1991' and under 'National Executive / Jaipur 1-3 February,
  1991', names no speaker and does not say that the taking over happened; the party's other compilations date the Jaipur
  meetings between 31 January and 3 February 1991. Neither Advani's end nor Joshi's start is recorded (check A13).
- **Thakre's 'I assume this office' is not a start.** It is in his speech to the National Council session of 3 and 4
  May 1998, which prints no day of delivery; the claim stores no structured date (check B1). He is observed on 27
  February 1999, the earliest single-day party styling found (check B11).
- **Laxman's end is accepted.** The office bearers' release of 14 March 2001 says they 'unanimously decided to accept the
  resignation with immediate effect': a same-day party record of the day the office ended, and this packet's only
  `until`. The atlas shows only "Observed on" for a holder with `attested_on`, so his note carries the end. The letter of
  13 March 2001 (recalled on 24 March), the release of 15 March and the National Executive's resolution of 24 March 2001
  are separate claims.
- **K. Jana Krishnamurthi's service from 14 March 2001 stays claims only (check B6).** The office bearers made him
  acting president on 14 March 2001. On 24 March 2001 the party heads his address 'National President' and he says the
  Executive has 'entrusted the responsibility of party presidentship' to him, but the same address places him in the
  Chair because of the office bearers' request, and no source states that the acting arrangement ended or that he was
  elected or appointed President; like the INC's interim presidency of 2019-2022, his service is claims only.
- **Naidu's start is accepted from the party journal (checks B3 and B5).** BJP Today's editorial for July 16-31, 2002
  says 'he took over from Shri Jana Krishnamurthi on July 1, his 53rd birth anniversary as the president'. The profile's
  'Ist July 2002 onwards President' is a retrospective span and not the basis, and Jana Krishnamurthi's end is not
  inferred from Naidu's start.
- **Naidu's 2004 end is declined.** On 18 October 2004 'With much reluctance, the resignation was accepted', with no day
  of effect; it is a claim.
- **Advani's 2004 start is declined (checks B3 and B4).** His statement of 20 October 2004 speaks of 'assuming the office
  of the President of the Bharatiya Janata Party two days ago', a recollection in passing, and CLAUDE-C01-15 check C1
  ruled that a later recollection of an assumption never dates a holder; the resolution of 18 October 2004 to appoint
  him, to be 'duly ratified' by the National Council, is an appointment. He is observed on 20 October 2004. The party
  journal's report that the National Council endorsed his election on 27 October 2004 and the release of 30 October,
  which names the National Executive, are separate claims, and the conflict is kept as printed (check B9).
- **Rajnath Singh's 2006 start is declined (check B8).** 'I am accepting the office of the National President of the
  Bharatiya Janata Party with all humility at my command' is in the present tense, 'I have taken over' is undated, and
  the responsibility placed on him 'On the conclusion of the august convention' (dated 28-30 December 2005 by the party)
  gives no day. He is observed on 2 January 2006.
- **Gadkari's profile dates no holder (checks C1 and C2).** The item page prints 'Friday, 18 December 2009' and the
  site's listing of 28 December 2009 gives 'Saturday, 19 December 2009'; neither is used. He is observed on 24 December
  2009 (his first press conference as President) and on 22 January 2013, the day he decided not to seek a second term,
  from the page's own title (check C3).
- **Rajnath Singh's 2013 observation is the page's date (check C4).** He was declared elected unopposed on 23 January 2013
  and styled 'Newly Elected President' that day, which are claims. The National Council met on 2 and 3 March 2013 and the
  Hindi text of his address is filed as 3 March, so 2 March 2013 is the page's date, not a printed day of delivery.
- **A re-election is not an observation (check C5).** Amit Shah's re-election reported on 24 January 2016 is a claim; he
  is observed on 2 February 2016 by the party's felicitation release.
- **No end from a handover (checks C6 and C9).** The party organ says Amit Shah 'handed over charge' to Nadda and Nadda
  'handed over the charge' to Nabin; each day is the successor's day of taking charge, and a handover statement is never
  an until.
- **Nadda's and Nabin's starts are accepted (checks C7 and C10).** Kamal Sandesh says Nadda 'took charge from the
  outgoing Party National President and Union Home Minister Shri Amit Shah on 20 January, 2020'. On 20 January 2026 the
  party's release quotes Nadda saying Nabin 'has assumed charge as the National President', and Kamal Sandesh says Nabin
  'formally assumed charge at the party headquarters in New Delhi on 20 January 2026'. The 2023 recollection of Nadda's
  taking charge is a claim.
- **Working presidencies are claims only.** Nadda was National Working President from 17 June 2019 and Nabin from 14
  and 15 December 2025; no holder of either name is dated inside those windows.
- **19 and 20 January 2026 (check C30).** The National Returning Officer's notice of 16 January fixed nominations,
  scrutiny, withdrawal and his press statement for 19 January and the official announcement, or a poll if needed, for 20
  January; his statement of 19 January announced that 'only 1 name, that of Shri Nitin Nabin, has been proposed'. The
  party organ's 'elected unopposed' on 19 January and its 'formal announcement' on 20 January 2026 are kept as separate
  claims.
- **Kamal Sandesh is a party record (check C32).** The magazine is printed and published on behalf of Dr. Mookerjee
  Smruti Nyas, but the party website links it from its own navigation and stores its issue PDFs under
  www.bjp.org/files/kamal-sandesh-documents/. Four stored issues are recorded, and the other four Kamal Sandesh sources
  are pre-cutoff Internet Archive captures of web posts (three WordPress API objects and one page), not live renderings
  (check C15).

### Date ledger

Each row is a separate dated fact with its own claim; two facts on one day stay two claims. "Observed" marks a holder's
`attested_on` claim, and "start" and "end" a holder's stated `from` or `until`; every other claim never feeds a holder.

| Date | Events | Claims |
|---|---|---|
| 8 Jan 1991 | Advani: in office | `in_rs_sahay_advani_president_bjp_rath_yatra_19910108` (observed) |
| 31 Jan 1991 | Joshi: taking over announced for the next day | `in_bjp_valedictory_joshi_taking_over_tomorrow_19910131` (claim) |
| 1 Feb 1991 | unnamed: acceptance; Advani: declined a third term | `in_bjp_nc_jaipur_new_president_accepts_presidentship_19910201` (claim), `in_bjp_nc_jaipur_advani_declined_third_term_19910201` (claim) |
| 10 Dec 1991 | Joshi: in office | `in_rs_narayanasamy_ekta_yatra_bjp_president_joshi_19911210` (observed) |
| 4 Mar 1992 | Joshi: in office again; Joshi: in office again | `in_rs_question_joshi_president_bjp_rocket_fire_19920304` (claim), `in_rs_jacob_answer_bjp_president_joshi_flag_hoisting_19920304` (claim) |
| 27 Feb 1993 | Joshi: in office again | `in_bjp_ne_resolution_assault_bjp_president_joshi_19930227` (claim) |
| 18 Jun 1993 | unnamed: election by the National Council stated; unnamed: name proposed; Joshi: predecessor reference | `in_bjp_nc_bangalore_elected_once_again_president_19930618` (claim), `in_bjp_nc_bangalore_joshi_proposed_name_19930618` (claim), `in_bjp_nc_bangalore_joshi_stewardship_predecessor_19930618` (claim) |
| 10 Nov 1995 | unnamed: re-election stated | `in_bjp_nc_mumbai_elected_president_yet_another_term_19951110` (claim) |
| 18 May 1997 | Advani: retrospective statement | `in_bjp_evolution_advani_president_swarna_jayanti_yatra_19970518` (claim) |
| 16 Jul 1997 | Advani: in office | `in_bjp_press_release_president_advani_19970716` (observed) |
| 11 Apr 1998 | unnamed: unnamed President in office; unnamed: choice to be declared; Advani: 'outgoing' | `in_bjp_ne_new_delhi_last_meeting_during_tenure_19980411` (claim), `in_bjp_advani_new_president_to_be_declared_next_week_19980411` (claim), `in_bjp_advani_outgoing_president_last_ne_19980411` (claim) |
| 2 May 1998 | unnamed: unnamed President in office; unnamed: handover statement; Thakre: election stated; Thakre: President-elect; unnamed: farewell | `in_bjp_ne_gandhinagar_i_as_president_19980502` (claim), `in_bjp_ne_gandhinagar_handover_announced_for_tomorrow_19980502` (claim), `in_bjp_ne_gandhinagar_thakre_elected_unanimously_19980502` (claim), `in_bjp_ne_gandhinagar_thakre_president_elect_19980502` (claim), `in_bjp_ne_gandhinagar_farewell_as_party_president_19980502` (claim) |
| 27 Feb 1999 | Thakre: in office | `in_bjp_thakre_styled_president_budget_statement_19990227` (observed) |
| 4 Jul 2000 | Thakre: in office again | `in_bjp_thakre_press_statement_as_president_20000704` (claim) |
| 2 Aug 2000 | Laxman: nominations filed; Thakre: predecessor reference | `in_bjp_laxman_nomination_papers_filed_20000802` (claim), `in_bjp_thakre_outgoing_president_present_20000802` (claim) |
| 31 Aug 2000 | Laxman: in office | `in_bjp_laxman_styled_president_maiden_press_conference_20000831` (observed) |
| 1 Sep 2000 | Laxman: in office again | `in_bjp_laxman_statement_as_president_20000901` (claim) |
| 9 Mar 2001 | Laxman: in office again | `in_bjp_laxman_press_statement_as_president_20010309` (claim) |
| 13 Mar 2001 | Laxman: resignation letter | `in_bjp_laxman_resignation_letter_dated_20010313_recalled` (claim) |
| 14 Mar 2001 | Laxman: resignation letter considered; Laxman: resignation accepted with immediate effect; Jana Krishnamurthi: acting president designated; Laxman: acceptance recalled | `in_bjp_office_bearers_consider_laxman_letter_20010314` (claim), `in_bjp_office_bearers_accept_laxman_resignation_immediate_effect_20010314` (end), `in_bjp_jana_krishnamurthi_designated_acting_president_20010314` (claim), `in_bjp_office_bearers_acceptance_recalled_20010314` (claim) |
| 15 Mar 2001 | Laxman: stepped down | `in_bjp_laxman_has_stepped_down_pending_enquiry_20010315` (claim) |
| 18 Mar 2001 | Jana Krishnamurthi: acting service | `in_bjp_jana_krishnamurthy_styled_president_20010318` (claim) |
| 24 Mar 2001 | Jana Krishnamurthi: acting service; Jana Krishnamurthi: entrustment by the National Executive stated; Laxman: resignation recorded | `in_bjp_jana_krishnamurthy_styled_national_president_ne_address_20010324` (claim), `in_bjp_ne_entrusted_presidentship_to_jana_krishnamurthy_20010324` (claim), `in_bjp_ne_resolution_laxman_resigned_as_president_20010324` (claim) |
| 29 Mar 2001 | Jana Krishnamurthi: acting service | `in_bjp_jana_krishnamurthi_styled_president_20010329` (claim) |
| 24 Jun 2002 | Jana Krishnamurthi: acting service | `in_bjp_national_president_jana_krishnamurthi_constitutes_committee_20020624` (claim) |
| 1 Jul 2002 | Naidu: assumption of charge stated; Jana Krishnamurthi: predecessor reference | `in_bjp_today_naidu_took_over_on_july_1_20020701` (start), `in_bjp_today_jana_krishnamurthi_predecessor_20020701` (claim) |
| 11 Jul 2002 | Naidu: in office again | `in_bjp_naidu_president_first_press_conference_20020711` (claim) |
| 3 Aug 2002 | unnamed: endorsement stated | `in_bjp_national_council_address_endorsement_stated_20020803` (claim) |
| 18 Oct 2004 | Naidu: in office again; Naidu: resignation offered; Naidu: resignation accepted (no day of effect); Advani: appointment, ratification to follow; Advani: assumption recalled | `in_bjp_naidu_chairs_meeting_as_party_president_20041018` (claim), `in_bjp_naidu_offers_resignation_20041018` (claim), `in_bjp_naidu_resignation_accepted_20041018` (claim), `in_bjp_advani_appointed_president_ratification_pending_20041018` (claim), `in_bjp_advani_assumed_office_two_days_ago_recalled_20041018` (claim) |
| 20 Oct 2004 | Advani: in office | `in_bjp_advani_statement_as_president_20041020` (observed) |
| 27 Oct 2004 | Advani: National Council endorsement; Naidu: farewell; Advani: election by the National Executive reported | `in_bjp_today_national_council_endorses_advani_election_20041027` (claim), `in_bjp_today_outgoing_president_naidu_farewell_20041027` (claim), `in_bjp_advani_elected_by_national_executive_20041027` (claim) |
| 30 Oct 2004 | Advani: in office again | `in_bjp_advani_constitutes_national_executive_20041030` (claim) |
| 8 Jun 2005 | Advani: resignation rejected | `in_bjp_parliamentary_board_rejects_advani_resignation_20050608` (claim) |
| 15 Jun 2005 | Advani: in office again | `in_bjp_advani_styled_president_after_resignation_episode_20050615` (claim) |
| 26 Dec 2005 | Advani: in office again; Advani: 'outgoing' | `in_bjp_advani_styled_president_opening_remarks_20051226` (claim), `in_bjp_advani_last_meeting_he_will_preside_20051226` (claim) |
| 2 Jan 2006 | Rajnath Singh: in office; Rajnath Singh: acceptance | `in_bjp_rajnath_singh_styled_national_president_20060102` (observed), `in_bjp_rajnath_singh_accepts_office_20060102` (claim) |
| 20 Jan 2006 | Rajnath Singh: in office again | `in_bjp_rajnath_singh_styled_president_national_council_20060120` (claim) |
| 19 Dec 2009 | Rajnath Singh: predecessor reference; Gadkari: listing date of the profile | `in_bjp_parliamentary_board_thanks_rajnath_singh_for_tenure_20091219` (claim), `in_bjp_listing_dates_gadkari_profile_20091219` (claim) |
| 24 Dec 2009 | Gadkari: in office | `in_bjp_gadkari_first_press_conference_as_president_20091224` (observed) |
| 18 Feb 2010 | Gadkari: in office again; Gadkari: election by the National Council stated | `in_bjp_gadkari_styled_national_president_indore_address_20100218` (claim), `in_bjp_gadkari_presidential_address_council_elected_him_20100218` (claim) |
| 16 Mar 2010 | Gadkari: in office again | `in_bjp_gadkari_announces_national_executive_as_president_20100316` (claim) |
| 22 Jan 2013 | Gadkari: in office; Gadkari: will not seek a second term | `in_bjp_gadkari_styled_national_president_statement_20130122` (observed), `in_bjp_gadkari_decides_not_to_seek_second_term_20130122` (claim) |
| 23 Jan 2013 | Rajnath Singh: election result; Rajnath Singh: declaration; Rajnath Singh: President-elect | `in_bjp_national_president_election_process_completed_20130123` (claim), `in_bjp_gehlot_declares_rajnath_singh_elected_unopposed_20130123` (claim), `in_bjp_rajnath_singh_styled_newly_elected_president_20130123` (claim) |
| 27 Jan 2013 | Gadkari: predecessor reference | `in_bjp_gadkari_styled_former_national_president_20130127` (claim) |
| 2 Mar 2013 | Rajnath Singh: in office | `in_bjp_rajnath_singh_presidential_address_national_council_20130302` (observed) |
| 9 Jul 2014 | Rajnath Singh: predecessor reference; Amit Shah: in office | `in_bjp_parliamentary_board_thanks_outgoing_president_rajnath_singh_20140709` (claim), `in_bjp_amit_shah_styled_national_president_profile_20140709` (observed) |
| 9 Aug 2014 | Amit Shah: in office again; Amit Shah: election by the National Council stated; Rajnath Singh: predecessor reference | `in_bjp_amit_shah_styled_president_national_council_address_20140809` (claim), `in_bjp_amit_shah_presidential_address_thanks_council_for_election_20140809` (claim), `in_bjp_amit_shah_recalls_rajnath_singh_tenure_20140809` (claim) |
| 24 Jan 2016 | Amit Shah: nominations filed; Amit Shah: election result | `in_bjp_amit_shah_nomination_filed_national_president_20160124` (claim), `in_bjp_amit_shah_re_elected_heading_20160124` (claim) |
| 2 Feb 2016 | Amit Shah: in office | `in_bjp_amit_shah_styled_national_president_felicitation_20160202` (observed) |
| 15 Oct 2016 | Amit Shah: in office again | `in_ks_amit_shah_styled_bjp_national_president_20161015` (claim) |
| 17 Jun 2019 | Amit Shah: in office; Nadda: Working President appointed; Amit Shah: request to be relieved; Amit Shah: to continue until the elections | `in_ks_amit_shah_styled_national_president_at_board_meeting_20190617` (observed), `in_ks_parliamentary_board_appoints_nadda_working_president_20190617` (claim), `in_ks_amit_shah_request_to_be_relieved_reported_20190617` (claim), `in_ks_amit_shah_continues_national_president_20190617` (claim) |
| 20 Jan 2020 | Nadda: election result; Nadda: assumption of charge stated; Nadda: nominations filed; Nadda: declaration; Amit Shah: handover statement; Nadda: retrospective statement; Gadkari: predecessor reference; Rajnath Singh: predecessor reference; Nadda: assumption recalled | `in_ks_nadda_elected_unopposed_11th_president_20200120` (claim), `in_ks_nadda_took_charge_20200120` (start), `in_ks_nadda_nomination_sole_candidate_20200120` (claim), `in_ks_radha_mohan_singh_declares_nadda_elected_20200120` (claim), `in_ks_amit_shah_hands_over_charge_20200120` (claim), `in_ks_life_sketch_nadda_elected_unopposed_20200120` (claim), `in_ks_gadkari_styled_former_party_president_20200120` (claim), `in_ks_rajnath_singh_styled_former_party_president_20200120` (claim), `in_ks_nadda_took_charge_20200120_recalled` (claim) |
| 17 Jan 2023 | Nadda: in office; Nadda: tenure extended; Nadda: extension announced | `in_bjp_nadda_styled_national_president_letter_20230117` (observed), `in_ks_national_executive_extends_nadda_tenure_to_june_2024_20230117` (claim), `in_ks_amit_shah_announces_extension_20230117` (claim) |
| 17 Feb 2024 | Nadda: in office again | `in_ks_nadda_styled_national_president_convention_20240217` (claim) |
| 14 Dec 2025 | Nabin: Working President appointed; Nabin: Working President appointment recalled | `in_ks_parliamentary_board_appoints_nabin_working_president_20251214` (claim), `in_ks_nabin_working_president_appointed_20251214_recalled` (claim) |
| 15 Dec 2025 | Nabin: charge as Working President; Nadda: in office | `in_ks_nabin_assumes_working_presidency_20251215` (claim), `in_ks_nadda_styled_national_president_20251215` (observed) |
| 16 Jan 2026 | unnamed: election schedule | `in_bjp_returning_officer_announces_president_election_schedule_20260116` (claim) |
| 19 Jan 2026 | Nabin: nominations filed; Nabin: scrutiny; Nabin: only name proposed; Nabin: election result | `in_bjp_nabin_nominations_filed_20260119` (claim), `in_bjp_nabin_nominations_valid_on_scrutiny_20260119` (claim), `in_bjp_returning_officer_announces_only_nabin_proposed_20260119` (claim), `in_ks_nabin_elected_unopposed_stated_20260119` (claim) |
| 20 Jan 2026 | Nabin: assumption of charge stated; Nadda: predecessor reference; Nabin: declaration; Nabin: assumption of charge stated; Nadda: handover statement; Rajnath Singh: predecessor reference; Amit Shah: predecessor reference; Gadkari: predecessor reference | `in_bjp_nadda_says_nabin_assumed_charge_today_20260120` (start), `in_bjp_nadda_styled_ex_national_president_20260120` (claim), `in_ks_laxman_declares_nabin_elected_20260120` (claim), `in_ks_nabin_assumes_charge_national_president_20260120` (start), `in_ks_nadda_hands_over_charge_20260120` (claim), `in_ks_former_national_president_rajnath_singh_20260120` (claim), `in_ks_former_national_president_amit_shah_20260120` (claim), `in_ks_former_national_president_nitin_gadkari_20260120` (claim) |
| 17 Aug 2026 | Nabin: in office | `in_ks_nabin_appoints_national_office_bearers_20260817` (observed) |
| undated | Advani: styled President in a narration; Advani: styled President (multi-day meeting); Joshi: styled President (multi-day meeting); Joshi: styled President (multi-day meeting); Advani: styled President (multi-day meeting); Joshi: predecessor reference; Advani: styled President (multi-day meeting); Advani: styled President (multi-day meeting); Joshi: taking over announced for the next day; unnamed: farewell; Advani: retrospective span; Advani: retrospective span; Advani: retrospective span; Advani: retrospective span; Joshi: retrospective span; Advani: styled President (multi-day meeting); Thakre: acceptance; Thakre: 'I assume this office' (no day); Thakre: styled President (multi-day meeting); Laxman: election by the National Council stated; Thakre: predecessor reference; Thakre: assumption recalled; Laxman: assumption recalled; Laxman: assumption recalled; Jana Krishnamurthi: acting responsibility accepted (recalled); Jana Krishnamurthi: assumption recalled; Naidu: retrospective span; Naidu: selection recalled; Advani: retrospective statement; unnamed: convention dates (context); Rajnath Singh: assumption recalled; Rajnath Singh: selection recalled; Gadkari: profile heading; Gadkari: selection recalled; Rajnath Singh: assumption recalled; Jana Krishnamurthi: retrospective span; Gadkari: retrospective span; Rajnath Singh: retrospective span; Amit Shah: selection recalled; Nadda: charge as Working President; Nadda: working presidency (retrospective); Nadda: working presidency (retrospective); Nadda: certificate presented; Amit Shah: handover statement; Gadkari: retrospective span; Rajnath Singh: retrospective span; Amit Shah: retrospective span; Nadda: retrospective span | `in_bjp_ne_resolution_narrates_president_advani_199004`, `in_bjp_ne_resolution_bjp_president_rath_yatra_199011`, `in_bjp_ne_resolution_our_president_joshi_ekta_yatra_199203`, `in_bjp_ne_resolution_release_joshi_president_bjp_199212`, `in_bjp_ne_resolution_our_president_advani_199312`, `in_bjp_ne_resolution_ex_president_joshi_199312`, `in_bjp_ne_resolution_party_president_advani_hawala_199602`, `in_bjp_ne_resolution_authorises_party_president_advani_199007`, `in_bjp_valedictory_joshi_to_assume_presidency_tomorrow_1991`, `in_bjp_outgoing_president_farewell_five_year_tenure_199101`, `in_bjp_profile_advani_president_until_199101`, `in_bjp_biodata_advani_president_19860509_to_1991`, `in_bjp_biodata_advani_president_199307_to_19980502`, `in_bjp_evolution_advani_national_president_except_1991_93`, `in_bjp_evolution_joshi_president_1991_1993`, `in_bjp_ne_resolution_gratitude_party_president_advani_199804`, `in_bjp_thakre_thanks_party_for_unanimous_election_199805`, `in_bjp_thakre_states_he_assumes_office_199805`, `in_bjp_thakre_speaks_as_president_national_council_199805`, `in_bjp_laxman_thanks_council_for_electing_him_fifth_president_200008`, `in_bjp_laxman_names_thakre_immediate_predecessor_200008`, `in_bjp_gs_report_thakre_took_over_at_gandhinagar_1998`, `in_bjp_laxman_took_over_reins_recalled_maiden_press_conference_200008`, `in_bjp_laxman_took_over_presidency_at_nagpur_session_200008`, `in_bjp_jana_krishnamurthy_accepted_acting_responsibility_recalled_2001`, `in_bjp_jana_krishnamurthi_taken_charge_recalled_2001`, `in_bjp_naidu_profile_span_ist_july_2002_onwards`, `in_bjp_naidu_entrusted_and_accepted_recalled_2002`, `in_bjp_today_vajpayee_fifth_time_advani_appointed_2004`, `in_bjp_rajat_jayanti_national_convention_dated_2005`, `in_bjp_rajnath_singh_taken_over_recalled_2006`, `in_bjp_rajnath_singh_responsibility_placed_after_convention_recalled_2005`, `in_bjp_gadkari_profile_heading_national_president_200912`, `in_bjp_gadkari_entrusted_new_responsibility_recalled_2009`, `in_bjp_rajnath_singh_took_over_again_recalled_2013`, `in_bjp_presidents_list_jana_krishnamurthy_2001_to_2002`, `in_bjp_presidents_list_gadkari_2010_to_2013`, `in_bjp_presidents_list_rajnath_singh_2013_to_present`, `in_ks_amit_shah_became_president_july_2014_recalled_2019`, `in_ks_nadda_takes_charge_working_president_2019`, `in_ks_life_sketch_nadda_working_president_onwards_2019`, `in_ks_life_sketch_nadda_working_president_period_2019_2020`, `in_ks_radha_mohan_singh_hands_certificate_to_nadda_2020`, `in_ks_amit_shah_today_nadda_takes_over_2020`, `in_bjp_presidents_list_gadkari_2010_2013`, `in_bjp_presidents_list_rajnath_singh_2005_2009_2013_2014`, `in_bjp_presidents_list_amit_shah_2014_2017_2017_2020`, `in_bjp_presidents_list_nadda_2020_present` (no structured date) |

Date conventions follow the stacked India packets: `attested_on` is the day of the observed event as the source dates
it, a page's issue date is `published_date`, and a retrospective statement that recalls a day is dated by that day;
multi-day meeting headings, spans, year- or month-only statements, undated lists and undated recollections carry no
structured date.

## Observations

### BJP-PRES-01 — The President when the period opens, and the 1991 change

Evidence: the party's National Executive resolutions of 1990 style Advani President under multi-day headings. At
Calcutta (6-8 April 1990) a resolution recalls that on 10 December 1989 'Bharatiya Janata Party President, Shri Lal
Krishna Advani was assured by the Prime Minister' (`in_bjp_ne_resolution_narrates_president_advani_199004`); at Madras
(21-23 July 1990) the party 'also authorises its Party President Shri L. K. Advani' to appoint a sub-committee
(`in_bjp_ne_resolution_authorises_party_president_advani_199007`); at New Delhi (9-10 November 1990) 'the BJP President
undertook Ram Rath Yatra' and the authorities 'arrested Shri Advani'
(`in_bjp_ne_resolution_bjp_president_rath_yatra_199011`). In the Rajya Sabha's written answers of 8 January 1991 the
Minister of State for Home Affairs, Shri Subodh Kant Sahay, says 'Shri L. K. Advani, President BJP' conducted the Rath
Yatra (`in_rs_sahay_advani_president_bjp_rath_yatra_19910108`). The outgoing President's farewell, printed twice and
naming no speaker, says 'Tomorrow, our renowned associate Dr. Murli Manohar Joshi would be taking over as President of
Bharatiya Janata Party' (`in_bjp_valedictory_joshi_taking_over_tomorrow_19910131`; the other printing is
`in_bjp_valedictory_joshi_to_assume_presidency_tomorrow_1991`). The address of 1 February 1991 at Jaipur says everyone
wished 'he should continue as our President for a third term' and 'like Shri Vajpayee, Shri Advani declined it'
(`in_bjp_nc_jaipur_advani_declined_third_term_19910201`). The party's 1996 profile ('which post he held until January
1991'), its 2003 bio-data ('from May 9, 1986 to 1991') and its history ('except for about two years from 1991-93') are
retrospective.

Decision: accepted in part. Advani is observed on 8 January 1991, the earliest single-day record adopted.

Limits: no record attests the holder on 1 January 1990 itself, and the 1990 resolutions store no structured date. No
record states the day Advani's presidency ended or Joshi's began: the farewell is prospective, its two printings carry
conflicting headings (31 January and 1-3 February 1991), and the address of 1 February names no speaker. The live party
site's former-president pages and the Lok Sabha records of 1990-1991 could not be reached.

### BJP-PRES-02 — Murli Manohar Joshi

Evidence: the unnamed address of 1 February 1991 at Jaipur thanks the party's workers 'who have honoured an ordinary
fellow-worker like me by entrusting to him the onerous responsibility of presidentship of the Party'
(`in_bjp_nc_jaipur_new_president_accepts_presidentship_19910201`). In the Rajya Sabha on 10 December 1991, under the item
headed 'REGARDING "EKTA YATRA" BY B.J.P. PRESIDENT', Shri V. Narayanasamy asks the Home Minister 'to immediately stop
this Ekta Yatra, by the BJP President, Dr. Murli Manohar Joshi'
(`in_rs_narayanasamy_ekta_yatra_bjp_president_joshi_19911210`). Two written answers of 4 March 1992 speak of 'the plane
carrying Dr. Murli Manohar Joshi, President, BJP' and, in Hindi, of 'भारतीय जनता पार्टी के अध्यक्ष डा० मुरलीमनोहर जोशी'
(`in_rs_question_joshi_president_bjp_rocket_fire_19920304`,
`in_rs_jacob_answer_bjp_president_joshi_flag_hoisting_19920304`). The party's resolutions of March 1992 ('our President,
Dr. Murli Manohar Joshi') and December 1992 ('Dr. Murli Manohar Joshi, President BJP') are under multi-day headings; its
resolution of 27 February 1993 speaks of 'The murderous assault on the BJP President, Dr. Murli Manohar Joshi'
(`in_bjp_ne_resolution_assault_bjp_president_joshi_19930227`). The address of 18 June 1993 says 'Under his stewardship
the party has made immense strides during the past two and a half years'
(`in_bjp_nc_bangalore_joshi_stewardship_predecessor_19930618`), and by 18-19 December 1993 he is 'Ex-President, Dr. MM
Joshi' (`in_bjp_ne_resolution_ex_president_joshi_199312`). The party's history says 'From 1991 to 1993, Dr. Murli Manohar
Joshi became the President of BJP'.

Decision: accepted in part. Joshi is observed on 10 December 1991, the earliest single-day record adopted that names and
styles him.

Limits: no record of his election, of the day he assumed the office or of the day it ended was found. The Rajya Sabha
item of 5 December 1991 and the written answer of 18 December 1991, which name no President, are leads.

### BJP-PRES-03 — Advani's return (1993)

Evidence: in the address printed under 'National Council / Bangalore 18 June, 1993' the speaker, whom the text does not
name, thanks the Council 'for having elected me once again President of this great Party' and Dr. Joshi, 'who proposed
my name for this high office' (`in_bjp_nc_bangalore_elected_once_again_president_19930618`,
`in_bjp_nc_bangalore_joshi_proposed_name_19930618`); at Mumbai on 10 November 1995 an unnamed speaker thanks the delegates
'for having elected me the President of this great Party for yet another term'
(`in_bjp_nc_mumbai_elected_president_yet_another_term_19951110`). The party's resolutions of 18-19 December 1993 ('our
President, Shri LK Advani'), 22-23 February 1996 ('our Party President', and separately 'Shri Advani') and 11-12 April
1998 ('the party President, Shri LK Advani') are under multi-day headings. The party's release headed 'Press Release by
President, Shri Lal Krishna Advani', dated 16 July 1997, has the author write 'As President of the BJP'
(`in_bjp_press_release_president_advani_19970716`). On 11 April and 2 May 1998 unnamed addresses speak of 'my tenure as
the Party President' and 'I as President, own full responsibility'; on 2 May the speaker says that 'tomorrow marks an
important day' as he hands over 'the baton of presidency to Shri Kushabhauji', and takes 'leave of you as Party President'
(`in_bjp_ne_gandhinagar_handover_announced_for_tomorrow_19980502`,
`in_bjp_ne_gandhinagar_farewell_as_party_president_19980502`). The 2003 bio-data gives 'again from July, 1993 to May 2,
1998'.

Decision: accepted in part. Advani is observed on 16 July 1997, the earliest single-day record adopted after his return.

Limits: no record states the day of the 1993 election or of his taking charge, or the day his office ended; the 1993,
1995 and 1998 addresses name no speaker (check A2), the handover is announced for the next day, and the bio-data is
retrospective. The party history's styling of him on the Swarna Jayanti Rath-Yatra of 18 May 1997 is retrospective
(`in_bjp_evolution_advani_president_swarna_jayanti_yatra_19970518`).

### BJP-PRES-04 — Kushabhau Thakre (1998) and Bangaru Laxman (2000)

Evidence: on 11 April 1998 the speaker named in the page heading as 'Advani' says the party's choice 'will be declared
next week' and calls himself 'the outgoing president' (`in_bjp_advani_new_president_to_be_declared_next_week_19980411`,
`in_bjp_advani_outgoing_president_last_ne_19980411`). The unnamed address of 2 May 1998 says the session will give the
party 'a new President, Shri Kushabhau Thakre, who has been elected unanimously to this post' and salutes the
'President-elect Shri Kushabhau Thakre' (`in_bjp_ne_gandhinagar_thakre_elected_unanimously_19980502`,
`in_bjp_ne_gandhinagar_thakre_president_elect_19980502`). In his speech to the session of '3rd and 4th May, 1998' Thakre
thanks the party 'for electing me unanimously' and says 'I assume this office by saluting the sacred memory of our
founder' (`in_bjp_thakre_states_he_assumes_office_199805`); in August 2000 the General Secretary's report recalls that he
'took over as the President of the Party from Shri Lal Krishna Advani' at the session of May 3 and 4, 1998
(`in_bjp_gs_report_thakre_took_over_at_gandhinagar_1998`). The party's page of 27 February 1999 is headed 'Statement on
Union Budget 1999-2000 issued by Shri Kushabhau Thakre, President, BJP'
(`in_bjp_thakre_styled_president_budget_statement_19990227`), and a press statement of 4 July 2000 is headed in the same
way. On 2 August 2000 'Eleven sets of nomination papers, all proposing the name of Shri Bangaru Laxman' were filed, with
'Shri Khushubhau Thakre, outgoing President' present (`in_bjp_laxman_nomination_papers_filed_20000802`,
`in_bjp_thakre_outgoing_president_present_20000802`). At the National Council session of 27-28 August 2000 Laxman thanks
the Council 'for electing me the Fifth President of the Bharatiya Janata Party' and calls Thakre 'my immediate
predecessor'. The party heads his statement of 31 August 2000 'Statement issued by Shri Bangaru Laxman, President BJP'
(`in_bjp_laxman_styled_president_maiden_press_conference_20000831`); in it he speaks of his 'maiden press conference after
taking over the reins of presidentship' at the session 'which concluded in Nagpur on August 28'. On 14 March 2001 the
available office bearers 'considered the letter from Shri Bangaru Laxman' and 'unanimously decided to accept the
resignation with immediate effect' (`in_bjp_office_bearers_consider_laxman_letter_20010314`,
`in_bjp_office_bearers_accept_laxman_resignation_immediate_effect_20010314`); the release of 15 March says he 'has stepped
down from the presidentship of the party', and the National Executive's resolution of 24 March 2001 speaks of his
'resigning from the post of president of the party'.

Decision: accepted in part. Thakre is observed on 27 February 1999. Laxman is observed on 31 August 2000, and his office
ended on 14 March 2001 by the office bearers' same-day release (stated end).

Limits: no record gives the day Thakre was declared elected or the day he took office, and his end is not inferred from
Laxman's election. No record gives Laxman's declaration or the day he assumed the office: the contents page of BJP Today
for August 16-31, 2000 lists an article on him as President-elect that is not archived (a lead).

### BJP-PRES-05 — K. Jana Krishnamurthi (2001) and M. Venkaiah Naidu (2002)

Evidence: the office bearers' meeting of 14 March 2001 'also decided that Shri K. Jana Krishnamurthi, Vice President will
be the acting president to head the party' (`in_bjp_jana_krishnamurthi_designated_acting_president_20010314`); a press
statement of 18 March 2001 is headed 'Press Statement issued by Shri K. Jana Krishnamurthy, President of BJP'
(`in_bjp_jana_krishnamurthy_styled_president_20010318`). On 24 March 2001 the party heads his address to the National
Executive 'Address of Shri K. Jana Krishnamurthy National President, BJP', and he says 'you have entrusted the
responsibility of party presidentship to me' (`in_bjp_jana_krishnamurthy_styled_national_president_ne_address_20010324`,
`in_bjp_ne_entrusted_presidentship_to_jana_krishnamurthy_20010324`); the same address recalls Laxman's letter of 13.3.2001
and that the office bearers 'asked me to shoulder the responsibility in his place'. On 29 March 2001 he says 'I have
taken charge of the Presidentship of the BJP', and on 24 June 2002 'The National President of the Party Shri K. Jana
Krishnamurthi has constituted an Election Management Committee'
(`in_bjp_national_president_jana_krishnamurthi_constitutes_committee_20020624`). BJP Today's editorial for July 16-31,
2002 says Naidu 'took over from Shri Jana Krishnamurthi on July 1, his 53rd birth anniversary as the president'
(`in_bjp_today_naidu_took_over_on_july_1_20020701`); the party's profile dated 1 July 2002 lists 'Ist July 2002 onwards
President, Bharatiya Janata Party' (`in_bjp_naidu_profile_span_ist_july_2002_onwards`), and on 11 July 2002 the party heads
his statement 'At his first press conference on becoming President of the BJP'. On 3 August 2002 an unnamed presidential
address thanks his fellow workers 'for having accepted and endorsed this decision'. On 18 October 2004 the meeting sat
'with Party President Shri M. Venkaiah Naidu in the chair', he 'offered to resign from his post for personal reasons',
and 'With much reluctance, the resignation was accepted' (`in_bjp_naidu_resignation_accepted_20041018`); on 27 October
2004 'Shri Venkaiah Naidu gave his farewell speech'.

Decision: accepted in part. Jana Krishnamurthi's service from 14 March 2001 is claims only; Naidu holds from 1 July 2002
(stated start).

Limits: no National Executive resolution electing or confirming Jana Krishnamurthi, no day of his assumption and no day
his office ended was found, and his acting service from 14 March 2001 is claims only. No record of the body that chose
Naidu, or of an election or ratification, was found, and no record states the day his office ended; his resignation was
accepted without a stated day of effect.

### BJP-PRES-06 — Advani (2004) and Rajnath Singh (2005)

Evidence: on 18 October 2004 the meeting 'unanimously resolved to appoint Shri L.K. Advani as the new President of the
Bharatiya Janata Party', and 'His appointment will be duly ratified at the meeting of the Party's National Council'
(`in_bjp_advani_appointed_president_ratification_pending_20041018`). The party heads his statement of 20 October 2004
'Statement issued by Shri L.K. Advani President', in which he speaks of his 'first press conference after assuming the
office of the President of the Bharatiya Janata Party two days ago' (`in_bjp_advani_statement_as_president_20041020`,
`in_bjp_advani_assumed_office_two_days_ago_recalled_20041018`). The party journal reports that on 27 October 2004 his
'election as Bharatiya Janata Party's National President was unanimously endorsed by BJP's supreme body - The National
Council' (`in_bjp_today_national_council_endorses_advani_election_20041027`), while the release of 30 October says he
'was elected the President of the BJP at the meeting of the Party's National Executive on October 27'
(`in_bjp_advani_elected_by_national_executive_20041027`). On 8 June 2005 'The meeting unequivocally rejected the
resignation' (`in_bjp_parliamentary_board_rejects_advani_resignation_20050608`); the party styles him President on 15
June and 26 December 2005, and on 26 December he says the meeting 'is going to be the last meeting of this Executive, and
the last meeting which I shall be presiding over' (`in_bjp_advani_last_meeting_he_will_preside_20051226`). The party
dates the Rajat Jayanti convention 'December 28, 29 & 30, 2005'. It heads a statement of 2 January 2006 'The statement
released by the BJP National President, Shri Rajnath Singh' (`in_bjp_rajnath_singh_styled_national_president_20060102`),
which opens 'I am accepting the office of the National President of the Bharatiya Janata Party with all humility at my
command'; on 20 January 2006 he says 'the onerous responsibility of Party's National President was placed at my
shoulders' 'On the conclusion of the august convention'.

Decision: accepted in part. Advani is observed on 20 October 2004 and Rajnath Singh on 2 January 2006.

Limits: the recollection of 20 October does not date a start, and the appointment of 18 October is not an assumption. No
record states the day Advani's office ended in 2005 or 2006, or records Rajnath Singh's election or its declaration; the
Chennai address of 16 September 2005 says nothing about stepping down, and the Parliamentary Board's statement of 10
June 2005 and Arun Jaitley's statement of 19 September 2005 were never captured.

### BJP-PRES-07 — Nitin Gadkari (2009) and Rajnath Singh (2013)

Evidence: the party's item headed 'Profile of Shri Nitin Gadkari BJP National President' is dated 'Friday, 18 December
2009' on its page and listed under 'Saturday, 19 December 2009'
(`in_bjp_gadkari_profile_heading_national_president_200912`, `in_bjp_listing_dates_gadkari_profile_20091219`); on 19
December 2009 the Parliamentary Board thanked Rajnath Singh 'for the leadership provided by him to the party during his
tenure as president' (`in_bjp_parliamentary_board_thanks_rajnath_singh_for_tenure_20091219`). On 24 December 2009 Gadkari
meets the national media 'in my first press conference as President of the BJP'
(`in_bjp_gadkari_first_press_conference_as_president_20091224`); on 18 February 2010 he tells the National Council that
'the party leadership and you unanimously elected me to occupy this position'. On 22 January 2013 the party titles a
statement 'Press statement issued by BJP National President, Shri Nitin Gadkari'
(`in_bjp_gadkari_styled_national_president_statement_20130122`), in which he has 'decided not to seek a second term as
the president of the BJP' (`in_bjp_gadkari_decides_not_to_seek_second_term_20130122`). On 23 January 2013 the National
Election Officer declares 'श्री राजनाथ सिंह जी' elected unopposed ('निर्विरोध निर्वाचित') for the session 2013-2015
(`in_bjp_gehlot_declares_rajnath_singh_elected_unopposed_20130123`), and the party titles his speech 'Acceptance speech
by Sh. Rajnath Singh Newly Elected President of BJP at New Delhi'; on 27 January 2013 the party calls Gadkari 'The former
BJP National President'. The party's page dated 2 March 2013 is headed 'Presidential speech by Shri Rajnath Singh at BJP
National Council Meeting at Talkatora Stadium' (`in_bjp_rajnath_singh_presidential_address_national_council_20130302`),
and the address speaks of 'aftertaking over the responsibility of the national president again' without a day.

Decision: accepted in part. Gadkari is observed on 24 December 2009 and 22 January 2013; Rajnath Singh is observed on 2
March 2013.

Limits: no record of the body that chose Gadkari, of the day of his selection or of the day he took charge, and no
record of the day Rajnath Singh took charge in 2013, was found; the party's lists ('2010 to 2013', '2013 to Present') are
retrospective. The Hindi text of the March 2013 address is a lead.

### BJP-PRES-08 — Amit Shah (2014, re-elected 2016)

Evidence: on 9 July 2014 the Parliamentary Board thanked the 'outgoing Party President, Shri Rajnath Singh', and the
party's release of the same day is headed 'Press : Profile of BJP National President, Shri Amit Shah'
(`in_bjp_parliamentary_board_thanks_outgoing_president_rajnath_singh_20140709`,
`in_bjp_amit_shah_styled_national_president_profile_20140709`). At the National Council of 9 August 2014 he speaks of
'my election as the BJP's National President'. The release of 24 January 2016 is headed 'Press : Shri Amit Shah
re-elected as BJP National President' (`in_bjp_amit_shah_re_elected_heading_20160124`), and its Hindi body says
nominations were filed for 'श्री अमित भाई शाह'; the release of 2 February 2016 is headed 'Press : Members from different
communities felicitating BJP National President, Shri Amit Shah at 11, Ashok Road'
(`in_bjp_amit_shah_styled_national_president_felicitation_20160202`). Kamal Sandesh reports him as 'BJP National President
Shri Amit Shah' on 15 October 2016 and, at the Parliamentary Board meeting of 17 June 2019, as 'Union Home Minister and BJP
National President Shri Amit Shah' (`in_ks_amit_shah_styled_national_president_at_board_meeting_20190617`), with his
request 'to relinquish his post' and the statement that he 'will continue as BJP President till the organizational
elections are over' (`in_ks_amit_shah_continues_national_president_20190617`). On 20 January 2020 'outgoing party
National President Shri Amit Shah handed over charge to Shri JP Nadda' (`in_ks_amit_shah_hands_over_charge_20200120`).

Decision: accepted in part. Amit Shah is observed on 9 July 2014, 2 February 2016 and 17 June 2019.

Limits: no party record of the decision naming him in 2014, its body or the day he took charge was found (the 2019 report
recalls only 'July 2014'); the 2016 re-election has no declaration text; the handover of 20 January 2020 is his
successor's day of taking charge and is not used as his end. The web archive's captures of www.bjp.org around 17 June
2019 and 20 January 2020 are 403 responses, so Kamal Sandesh is the only party record of those transitions.

### BJP-PRES-09 — J. P. Nadda

Evidence: Kamal Sandesh reports that the party 'appointed former Union Health Minister Shri Jagat Prakash Nadda as its
National Working President on 17 June, 2019' (`in_ks_parliamentary_board_appoints_nadda_working_president_20190617`). The
issue of 01-15 February, 2020 reports that he 'was elected unopposed as the 11th National President of BJP' and 'took
charge from the outgoing Party National President and Union Home Minister Shri Amit Shah on 20 January, 2020'
(`in_ks_nadda_elected_unopposed_11th_president_20200120`, `in_ks_nadda_took_charge_20200120`), and that 'The election
in-Charge and former Union Minister Shri Radha Mohan Singh made the official declaration of the election'
(`in_ks_radha_mohan_singh_declares_nadda_elected_20200120`); its life sketch lists '17 June, 2019 - 20th January, 2020: BJP
National Working President'. The party's page dated 17 January 2023 is headed 'Letter : Hon'ble BJP National President
Shri J.P. Nadda to the Party Karyakartas' (`in_bjp_nadda_styled_national_president_letter_20230117`), and the issue of
01-15 February, 2023 reports that on 17 January 2023 his 'tenure was extended unanimously till June 2024'
(`in_ks_national_executive_extends_nadda_tenure_to_june_2024_20230117`). He is 'BJP National President Shri Jagat Prakash
Nadda' at the National Convention of 17 February 2024 and 'BJP National President and Union Minister Shri JP Nadda' on 15
December 2025 (`in_ks_nadda_styled_national_president_20251215`).

Decision: accepted in part. Nadda holds from 20 January 2020 (stated start) and is observed on 17 January 2023 and 15
December 2025.

Limits: no further extension after June 2024 and no amendment of the term was found; the working presidency is claims
only; his handover of 20 January 2026 is never an until. The party's release of 25 December 2025, which still styles him
National President, is a lead.

### BJP-PRES-10 — The President at the cutoff

Evidence: the issue of 16-31 December, 2025 reports that on 14 December 2025 the Parliamentary Board appointed Nitin Nabin
National Working President and that he 'formally assumed charge as the National Working President at the party
headquarters in New Delhi on 15 December 2025' (`in_ks_parliamentary_board_appoints_nabin_working_president_20251214`,
`in_ks_nabin_assumes_working_presidency_20251215`). The National Returning Officer's notice dated 16 January 2026 fixes the
schedule (`in_bjp_returning_officer_announces_president_election_schedule_20260116`), and his statement of 19 January 2026
records the nominations, their scrutiny and that 'only 1 name, that of Shri Nitin Nabin, has been proposed'
(`in_bjp_nabin_nominations_filed_20260119`, `in_bjp_nabin_nominations_valid_on_scrutiny_20260119`,
`in_bjp_returning_officer_announces_only_nabin_proposed_20260119`). On 20 January 2026 the party's release quotes Nadda
saying Nabin 'has assumed charge as the National President of the world's largest party'
(`in_bjp_nadda_says_nabin_assumed_charge_today_20260120`), and the issue of 16-31 January, 2026 says he 'formally assumed
charge at the party headquarters in New Delhi on 20 January 2026'
(`in_ks_nabin_assumes_charge_national_president_20260120`), that 'Dr. K Laxman, made the official declaration of the
election' and that Nadda 'handed over the charge'. Kamal Sandesh's page captured on 4 September 2026 says 'Bharatiya
Janata Party Hon'ble National President Shri Nitin Nabin is pleased to appoint' the new national office bearers 'on 17
August 2026' (`in_ks_nabin_appoints_national_office_bearers_20260817`).

Decision: accepted. Nabin holds from 20 January 2026 (stated start) and is observed on 17 August 2026, before the cutoff.

Limits: the working presidency is claims only. A Kamal Sandesh post on a meeting of 1 September 2026 exists only as a
rendering modified after the cutoff and is a lead, as are the party's releases of 22 and 26 January 2026. Kamal
Sandesh's page of the appointments prints 'Published on: 16 Aug, 2026', a day before the appointment day in its text,
and was modified on 2 September 2026 before its only capture; the day is kept as the text prints it.

## Sources added

| Source ID | What | Provenance |
|---|---|---|
| `in_bjp_elib_party_document_vol5_political_resolutions` | Bharatiya Janata Party, Party Document Vol-5: Political Resolutions (BJP e-Library item 123456789/264, 'BJP-Political Resolutions', file Untitled-2.pdf) | official file, BJP e-Library (item 123456789/264); PDF pages 136, 137, 202, 203, 218, 220, 242, 246, 247, 268, 292 read |
| `in_bjp_elib_party_document_vol6_economic_resolutions` | Bharatiya Janata Party, Party Document Vol-6: Economic Resolutions (BJP e-Library item 123456789/265, 'BJP-Economics Resolutions', file BJP-Economics Resolutions.pdf) | official file, BJP e-Library (item 123456789/265); PDF pages 163, 189, 196, 205, 209 read |
| `in_rs_written_answers_19910108_usq1586_rath_yatra` | Rajya Sabha Debates, 8 January 1991 (Session 156): Written Answers, Unstarred Question 1586 'Rath Yatra', col. 324 (store file IQ_156_08011991_U1586_p324.pdf) | official file, Rajya Sabha debates store; PDF page 1 read |
| `in_bjp_elib_party_document_vol3_presidential_speeches_part2` | Bharatiya Janata Party, Party Document Vol-3: Presidential Speeches (Part-II), sections headed 'DR. MM JOSHI', 'SHRI LK ADVANI' and 'SHRI ATAL BIHARI VAJPAYEE' (BJP e-Library item 123456789/248, file DR. MM JOSHI.pdf) | official file, BJP e-Library (item 123456789/248); PDF pages 1, 3, 28, 31, 32, 73, 74, 85, 203, 204 read |
| `in_bjp_org_advani_profile_19961019` | Bharatiya Janata Party website, 'Shri. L.K. Advani - A Profile' (bjp.org/bjp/profiles/advani-1.html), Internet Archive capture of 19 October 1996 | capture 1996-10-19 of www.bjp.org |
| `in_bjp_org_lka_profile_biodata_20030317` | Bharatiya Janata Party website, 'Shri L.K Advani : a profile' with Bio-Data (bjp.org/leader/lka-profile.htm), Internet Archive capture of 17 March 2003 | capture 2003-03-17 of www.bjp.org |
| `in_bjp_elib_party_document_vol10_evolution_of_bjp` | Bharatiya Janata Party, Party Document Vol-10: Evolution of BJP (1980-2005), by Vijay Kumar Malhotra and J.C. Jaitli (BJP e-Library item 123456789/275, file Evolution of BJP - Full.pdf) | official file, BJP e-Library (item 123456789/275); PDF pages 20, 21, 31, 40 read |
| `in_rs_debate_19911210_ekta_yatra_bjp_president` | Rajya Sabha Debates, 10 December 1991 (Session 161): 'Regarding "Ekta Yatra" by B.J.P. President', cols 223-228 (store file ID_161_10121991_01_p223-228_1.pdf) | official file, Rajya Sabha debates store; PDF pages 1, 2, 3 read |
| `in_rs_written_answers_19920304_usq1060_rocket_fire` | Rajya Sabha Debates, 4 March 1992 (Session 162): Written Answers, Unstarred Question 1060 'Rocket Fire on Plane Carrying BJP President', cols 181-182 (store file IQ_162_04031992_U1060_p181-182.pdf) | official file, Rajya Sabha debates store; PDF page 1 read |
| `in_rs_written_answers_19920304_usq1067_flag_hoisting` | Rajya Sabha Debates, 4 March 1992 (Session 162): Written Answers, Unstarred Question 1067 (Hindi) 'भारतीय जनता पार्टी के अध्यक्ष द्वारा श्रीनगर में ध्वज फहराया जाना', cols 187-189 (store file IQ_162_04031992_U1067_p188-189.pdf) | official file, Rajya Sabha debates store; PDF pages 1, 2 read |
| `in_bjp_elib_party_document_vol2_presidential_speeches_part1` | Bharatiya Janata Party, Party Document Vol-2: Presidential Speeches (Part-I), with the section headed 'SHRI LK ADVANI' for 1993-1998 (BJP e-Library item 123456789/247, file Lal Krishna Advani.pdf) | official file, BJP e-Library (item 123456789/247); PDF pages 296, 298, 310, 311, 316, 372, 438 read |
| `in_bjp_org_swarna_jayanti_press_release_19970716` | Bharatiya Janata Party website, 'Swarna Jayanti Rath Yatra: Press Release by President, Shri Lal Krishna Advani, July 16, 1997, New Delhi' (bjp.org/leader/sjry-press.html), Internet Archive capture of 19 May 1998 | capture 1998-05-19 of bjp.org |
| `in_bjp_elib_party_document_vol4_foreign_policy_resolutions` | Bharatiya Janata Party, Party Document Vol-4: Foreign Policy and Other Resolutions (BJP e-Library item 123456789/266, file Foreign Policy, Defence & International Commerce.pdf) | official file, BJP e-Library (item 123456789/266); PDF pages 81, 82, 223, 225 read |
| `in_bjp_site_advani_opening_remarks_ne_19980411` | Advani's opening remarks, BJP National Executive meeting held on 11 April 1998 (bjp.org news page nr3a13.htm) | capture 1999-02-02 of bjp.org |
| `in_bjp_site_thakre_presidential_speech_gandhinagar_199805` | Presidential speech of Kushabhau Thakre, BJP National Council session, Gandhi Nagar (Gujarat), 3 and 4 May 1998 (bjp.org statement page stmay7.htm, page 1 of 3) | capture 1998-05-19 of www.bjp.org |
| `in_bjp_site_thakre_budget_statement_19990227` | Statement on Union Budget 1999-2000 issued by Shri Kushabhau Thakre, President, BJP, 27 February 1999 (bjp.org news page feb2799.htm) | capture 2000-09-30 of www.bjp.org |
| `in_bjp_site_thakre_press_statement_20000704` | Press Statement issued by Shri Kushabhau Thakre, President, BJP, 4 July 2000 (bjp.org news page 'Press (4-7-00).htm') | capture 2000-10-08 of bjp.org |
| `in_bjp_site_laxman_nomination_20000802` | Bangaru Laxman Nominated for Presidentship, 2 August 2000 (bjp.org news page Aug0200a.htm) | capture 2000-10-08 of www.bjp.org |
| `in_bjp_site_laxman_presidential_address_nagpur_200008` | Presidential Address by Shri Bangaru Laxman, National Council Session, 27-28 August 2000, Nagpur (bjp.org news page Aug3100.htm) | capture 2001-03-03 of www.bjp.org |
| `in_bjp_site_general_secretary_report_nagpur_200008` | General Secretary's Report: M. Venkaiah Naidu, BJP National Council Session, August 27-28, 2000 (bjp.org news page Aug3100c.htm) | capture 2001-04-07 of bjp.org |
| `in_bjp_site_laxman_statement_20000831` | Statement issued by Shri Bangaru Laxman, President BJP, 31 August 2000 (bjp.org news page Aug3100d.htm) | capture 2001-03-04 of bjp.org |
| `in_bjp_site_laxman_statement_20000901` | Statement issued by Shri Bangaru Laxman, President BJP, 1 September 2000 (bjp.org news page Sep0100.htm) | capture 2000-12-17 of bjp.org |
| `in_bjp_site_laxman_press_statement_20010309` | Press Statement issued by Shri Bangaru Laxman, President, BJP, 9 March 2001 (bjp.org news page Mar0901.htm) | capture 2001-04-07 of bjp.org |
| `in_bjp_site_office_bearers_press_release_20010314` | BJP Press Release of 14 March 2001: meeting of the available central office bearers (bjp.org news page Mar1401.htm) | capture 2001-04-07 of www.bjp.org |
| `in_bjp_site_press_release_20010315` | BJP Press Release of 15 March 2001 on the political situation and the Tehelka tapes (bjp.org news page Mar1501.htm) | capture 2001-04-07 of www.bjp.org |
| `in_bjp_site_jana_krishnamurthy_press_statement_20010318` | Press Statement issued by Shri K. Jana Krishnamurthy, President of BJP, 18 March 2001 (bjp.org news page Mar1801.htm) | capture 2001-04-07 of bjp.org |
| `in_bjp_site_jana_krishnamurthy_ne_address_20010324` | Opening Remarks: Shri K. Jana Krishnamurthy, President, BJP, National Executive meeting, Parliament Annexe, New Delhi, 24 March 2001 (bjp.org news page Mar2401.htm) | capture 2001-08-16 of www.bjp.org |
| `in_bjp_site_ne_resolution_political_situation_20010324` | Resolution adopted on Current Political Situation, BJP National Executive, 24 March 2001 (bjp.org news page Mar2401c.htm) | capture 2001-04-07 of bjp.org |
| `in_bjp_site_jana_krishnamurthi_press_statement_20010329` | Press statement issued by Shri K. Jana Krishnamurthi, President, BJP, 29 March 2001 (bjp.org news page Mar2901.htm) | capture 2001-04-07 of bjp.org |
| `in_bjp_site_shastri_press_statement_20020624` | Press Statement issued by Shri Sunil Shastri, General Secretary, BJP, 24 June 2002 (bjp.org news page 'June 2402.htm') | capture 2003-01-18 of www.bjp.org |
| `in_bjp_site_naidu_profile_20020701` | Profile of Shri M. Venkaiah Naidu, 'Presidents' (sic) Bharatiya Janata Party, dated 1 July 2002 (bjp.org news page 'July 0102.htm') | capture 2004-08-25 of www.bjp.org |
| `in_bjp_site_naidu_first_press_conference_20020711` | Statement issued by Shri M. Venkaiah Naidu, President of the BJP, at his first press conference on becoming President, New Delhi, 11 July 2002 (bjp.org news page 'July 1102.htm') | capture 2003-01-15 of www.bjp.org |
| `in_bjp_today_editorial_20020716` | BJP Today, July 16-31, 2002, Vol. 11, No. 14: 'Letter from the Editor' by Arabinda Ghose (bjp.org page today/July 0202a.htm) | capture 2002-11-21 of www.bjp.org |
| `in_bjp_site_naidu_national_council_address_20020803` | Presidential Address at the Meeting of the National Council of Bharatiya Janata Party, 3 August 2002 (bjp.org news page Aug0302.htm) | capture 2002-10-18 of www.bjp.org |
| `in_bjp_site_press_statement_naidu_resignation_20041018` | Press statement on Shri Venkaiah Naidu's resignation, 18 October 2004 (bjp.org press page oct_1804.htm) | capture 2004-12-14 of www.bjp.org |
| `in_bjp_site_advani_statement_20041020` | Statement issued by Shri L.K. Advani, President, at a press conference in New Delhi on 20 October 2004 (bjp.org press page oct_2004.htm) | capture 2004-12-16 of www.bjp.org |
| `in_bjp_today_national_council_report_20041027` | BJP Today, November 1-15, 2004, Vol. 13, No. 21: 'Advaniji's election historic : Vajpayee', a report on the National Council meeting (bjp.org page today/Nov_0104/Nov_1_p_29.htm) | capture 2004-12-22 of www.bjp.org |
| `in_bjp_site_press_release_office_bearers_20041030` | BJP Central Office Press Release of 30 October 2004: new National Executive and office bearers (bjp.org press page Office_Bearers.htm) | capture 2004-11-01 of www.bjp.org |
| `in_bjp_site_resolution_advani_resignation_20050608` | Resolution passed by Central Office bearers and Parliamentary Board Meeting, 8 June 2005 (bjp.org press page june_0805.htm) | capture 2007-08-07 of www.bjp.org |
| `in_bjp_site_advani_speech_20050615` | Speech by Shri L.K. Advani, President, BJP & Leader of the Opposition (Lok Sabha), book release, New Delhi, 15 June 2005 (bjp.org press page june_1505b.htm) | capture 2006-10-04 of www.bjp.org |
| `in_bjp_site_advani_opening_remarks_ne_20051226` | Opening Remarks by BJP President Shri L.K. Advani at National Executive Meeting in Mumbai, 26 December 2005 (bjp.org press page dec_2605.htm) | capture 2006-06-28 of www.bjp.org |
| `in_bjp_site_rajat_jayanti_sandesh_20051230` | Rajat Jayanti Sandesh, Bharatiya Janata Party National Convention, Mumbai, dated 30 December 2005 (bjp.org press page dec_3005_sandesh.htm) | capture 2006-06-18 of www.bjp.org |
| `in_bjp_site_rajnath_singh_statement_20060102` | The statement released by the BJP National President, Shri Rajnath Singh, 2 January 2006 (bjp.org press page jan_0206a_p.htm) | capture 2006-10-04 of www.bjp.org |
| `in_bjp_site_rajnath_singh_national_council_address_20060120` | Presidential Address by Shri Rajnath Singh, President, Bharatiya Janata Party, National Council, New Delhi, 20 January 2006 (bjp.org press page jan_2006_p.htm) | capture 2006-12-06 of www.bjp.org |
| `in_bjp_site_profile_gadkari_national_president_20091218` | Profile of Shri Nitin Gadkari BJP National President (bjp.org Press Releases item content/view/3095, page dated Friday, 18 December 2009) | capture 2009-12-25 of www.bjp.org |
| `in_bjp_site_parliamentary_board_resolution_rajnath_singh_20091219` | Resolution passed by BJP Parliamentary Board (bjp.org Press Releases item content/view/3097, dated Saturday, 19 December 2009) | capture 2009-12-23 of www.bjp.org |
| `in_bjp_site_gadkari_first_press_conference_as_president_20091224` | Press Statement issued by Shri Nitin Gadkari at his first press conference as National President of the BJP in New Delhi (bjp.org item content/view/3103, dated Thursday, 24 December 2009) | capture 2010-04-09 of www.bjp.org |
| `in_bjp_site_press_release_listing_20091228` | Press Releases listing of the bjp.org website, as captured on 28 December 2009 (content/category/24/38/394/) | capture 2009-12-28 of www.bjp.org |
| `in_bjp_site_gadkari_presidential_address_national_council_indore_20100218` | Presidential Address By Shri Nitin Gadkari at National Council meeting of the BJP, Indore (Madhya Pradesh), 18 February 2010 (bjp.org item content/view/3162) | capture 2010-02-21 of www.bjp.org |
| `in_bjp_site_gadkari_announces_national_executive_20100316` | BJP President Shri Nitin Gadkari announces National Office Bearers, Members, Permanent Invitees, Special Invitees and Parliamentary Board (bjp.org press release dated Tuesday, 16 March 2010; as still carried on the site and captured 4 February 2016) | capture 2016-02-04 of www.bjp.org |
| `in_bjp_site_gadkari_statement_no_second_term_20130122` | Press statement issued by BJP National President, Shri Nitin Gadkari (bjp.org Press Releases item id 8506, dated Tuesday, 22 January 2013) | capture 2013-01-25 of www.bjp.org |
| `in_bjp_site_national_election_officer_declaration_rajnath_singh_20130123` | BJP Organisational Elections 2013: Election of the National President, declaration by Thawarchand Gehlot, National Election Officer, dated 23 January 2013 (Hindi PDF hindi_thawarji_jan_23_2013.pdf on bjp.org) | capture 2013-10-21 of www.bjp.org; PDF page 1 read |
| `in_bjp_site_rajnath_singh_acceptance_speech_newly_elected_20130123` | Acceptance speech by Sh. Rajnath Singh Newly Elected President of BJP at New Delhi (bjp.org Speeches item id 8517, dated Wednesday, 23 January 2013) | capture 2013-02-02 of www.bjp.org |
| `in_bjp_site_gadkari_styled_former_president_20130127` | Press release by Former BJP President, Shri Nitin Gadkari (bjp.org Press Releases item id 8516, dated Sunday, 27 January 2013) | capture 2013-02-02 of www.bjp.org |
| `in_bjp_site_rajnath_singh_presidential_address_national_council_20130302` | Speech: Sh. Rajnath Singh at BJP National Council Meeting at Talkatora Stadium (bjp.org item id 8592, dated Saturday, 02 March 2013; National Council 2-3 March 2013) | capture 2013-03-10 of www.bjp.org |
| `in_bjp_site_bjp_presidents_1980_2013_list_20140117` | BJP Presidents from 1980 to 2013 (bjp.org/leadership/bjp-presidents, as captured 17 January 2014) | capture 2014-01-17 of www.bjp.org |
| `in_bjp_site_parliamentary_board_resolution_rajnath_singh_20140709` | Press : Resolution passed in BJP Parliament Board (bjp.org press release dated Wednesday, 09 July 2014) | capture 2014-07-13 of www.bjp.org |
| `in_bjp_site_profile_national_president_amit_shah_20140709` | Press : Profile of BJP National President, Shri Amit Shah (bjp.org press release dated Wednesday, 09 July 2014) | capture 2014-07-12 of www.bjp.org |
| `in_bjp_site_amit_shah_presidential_address_national_council_20140809` | Presidential speech by Shri Amit Shah at BJP National Council Meeting at New Delhi (bjp.org release dated Saturday, 09 August 2014) | capture 2014-08-12 of www.bjp.org |
| `in_bjp_site_amit_shah_re_elected_national_president_20160124` | Press : Shri Amit Shah re-elected as BJP National President (bjp.org press release dated Sunday, 24 January 2016; body in Hindi) | capture 2016-01-25 of www.bjp.org |
| `in_bjp_site_amit_shah_felicitation_20160202` | Press : Members from different communities felicitating BJP National President, Shri Amit Shah at 11, Ashok Road (bjp.org press release dated Tuesday, 02 February 2016; body in Hindi) | capture 2016-02-03 of www.bjp.org |
| `in_ks_amit_shah_valmiki_jayanti_address_20161015` | Everybody in society can seek inspiration from Maharshi Valmiki : Amit Shah (Kamal Sandesh web post 13, WordPress API object as captured on 6 May 2026) | capture 2026-05-06 of kamalsandesh.org |
| `in_ks_nadda_appointed_working_president_20190617` | Jagat Prakash Nadda appointed as Working President of BJP (Kamal Sandesh web post 7435, WordPress API object as captured on 7 November 2025) | capture 2025-11-07 of kamalsandesh.org |
| `in_kamal_sandesh_vol15_no03_20200201` | Kamal Sandesh, Vol. 15, No. 03, 01-15 February, 2020 (Fortnightly): 'JP Nadda Elected Unopposed as BJP National President' (issue PDF KS_ENG_Feb 2020_1.pdf on www.bjp.org) | capture 2025-05-11 of www.bjp.org; PDF pages 1, 3, 6, 7, 8, 9, 12 read |
| `in_bjp_site_nadda_letter_to_karyakartas_20230117` | Letter : Hon'ble BJP National President Shri J.P. Nadda to the Party Karyakartas (bjp.org press release page dated 17-01-2023) | capture 2023-01-17 of www.bjp.org |
| `in_kamal_sandesh_vol18_no03_20230201` | Kamal Sandesh, Vol. 18, No. 03, 01-15 February, 2023 (Fortnightly): 'Bjp National Executive unanimously extends Party National President's tenure' (issue PDF KS_ENG_Feb 2023_1_Web.pdf on www.bjp.org) | capture 2024-08-07 of www.bjp.org; PDF pages 1, 8 read |
| `in_ks_nadda_address_national_convention_20240217` | Under the leadership of PM Modi, nation is advancing on the path of development: JP Nadda (Kamal Sandesh web post 30863, WordPress API object as captured on 6 May 2026) | capture 2026-05-06 of kamalsandesh.org |
| `in_bjp_site_bjp_presidents_list_20251120` | BJP Presidents (www.bjp.org/bjp-presidents; the party's list of its presidents with years, as captured by Arquivo.pt on 20 November 2025) | Arquivo.pt capture 2025-11-20 of www.bjp.org |
| `in_kamal_sandesh_vol20_no24_20251216` | Kamal Sandesh, Vol. 20, No. 24, 16-31 December, 2025 (Fortnightly), 'Published on: 19 December, 2025': 'Nitin Nabin Assumes Charge as the National Working President of BJP' (issue PDF KS_ENG_Dec 2025_2_Web.pdf on www.bjp.org) | capture 2026-01-30 of www.bjp.org; PDF pages 1, 2, 6, 36 read |
| `in_bjp_site_returning_officer_notice_20260116` | Notice of Dr. K. Laxman, M.P. Rajya Sabha, National Returning Officer, dated 16.01.2026, 'संगठन पर्व - 2024': the schedule of the election of the National President (Hindi PDF on www.bjp.org) | capture 2026-01-16 of www.bjp.org; PDF page 1 read |
| `in_bjp_site_returning_officer_statement_20260119` | Press Statement of Dr. K. Laxman, National Returning Officer, Sangathan Parv, 19th January, 2026 (Monday) (PDF on www.bjp.org) | capture 2026-01-19 of www.bjp.org; PDF page 1 read |
| `in_bjp_site_nadda_felicitation_speech_20260120` | Salient points of speech of former BJP National President Shri J.P. Nadda at the felicitation ceremony for the Newly Elected BJP National President Shri Nitin Nabin (bjp.org press release page dated 20-01-2026) | capture 2026-01-20 of www.bjp.org |
| `in_kamal_sandesh_vol21_no02_20260116` | Kamal Sandesh, Vol. 21, No. 02, 16-31 January, 2026 (Fortnightly), 'Published on: 24 January, 2026': 'Nitin Nabin Elected Unopposed as BJP National President' (issue PDF KS_ENG_Jan 2026_2.pdf on www.bjp.org) | capture 2026-02-03 of www.bjp.org; PDF pages 1, 6, 7, 8, 9, 36 read |
| `in_ks_national_president_appoints_office_bearers_20260817` | BJP National President appoints the New National Office Bearers (Kamal Sandesh web page, as captured on 4 September 2026) | capture 2026-09-04 of kamalsandesh.org |

## Response identities and stability checks

The reviewer re-downloads every recorded response and compares its byte count and SHA-256, so every identity here was
downloaded at least three times with identical bytes, the first and last at least 30 minutes apart, and no identity is a
page generated per request, a growing listing, a search or a live WordPress API rendering. How each was established:

- **Raw Internet Archive captures (63).** Fetched in the `id_` form with curl's default User-Agent,
  `Accept-Encoding: identity` and without `--compressed`; every body was served with no Content-Encoding, so each recorded
  identity is the uncompressed archived body, and each recorded `id_` URL answers 200 without a redirect. All capture
  timestamps are before the cutoff. They are the party website's pages of 1996-2016 and 2023-2026, the party's stored
  PDFs (the 2013 declaration and the 2026 notice and statement of the National Returning Officer), BJP Today's pages of
  2002 and 2004, and the party organ's issue PDFs stored on www.bjp.org and pre-cutoff captures of its web posts.
- **Arquivo.pt (1).** The capture of 20 November 2025 of www.bjp.org/bjp-presidents, fetched in the `id_` form
  in the same way.
- **The Bharatiya Janata Party's e-Library (6).** Static DSpace bitstreams on library.bjp.org, served without
  Content-Encoding and with a fixed Last-Modified (8 November 2016 for Vol-2 and Vol-3; 31 March 2017 for Vol-4, Vol-5,
  Vol-6 and Vol-10).
- **The Rajya Sabha Secretariat's debates store (4, and one date anchor).** The official host CLAUDE-C01-11 already
  records; each file's ETag equals the MD5 of its body and its Last-Modified is of 29 July 2026. The store's query sets
  only the served content type and file name. The date anchor (`IQ_156_08011991_U1587_p324-325.pdf`, 180,594 bytes,
  SHA-256 `5a201ebcca9a545a5ecc20d805ad7ccb08d1fbb714f40e39197384aab6794baa`) was downloaded by this packet at
  01:40:04Z, 02:38:57Z and 02:50:45Z on 28 September 2026.
- **Times.** The research parts downloaded the dossier responses between 01:48Z and 04:48Z on 26 September 2026, at
  least twice each; the checks downloaded them again between 02:46Z on 26 September and 01:26Z on 28 September 2026,
  including every record they added; this packet downloaded every recorded response twice more on 28 September 2026,
  between 01:39Z and 02:55Z, each pair at least 30 minutes apart, and the date anchor three times. The large copies were
  deleted from scratch after hashing.

| Source | Bytes | SHA-256 | Downloads with identical bytes |
|---|---:|---|---|
| `in_bjp_elib_party_document_vol5_political_resolutions` | 3,810,049 | `33ddad0dcd6f64726a041646440ef66379ff6c88cdc4ee26cf0edb85c20022f5` | research 01:49Z, 02:16Z, 02:19Z (26 Sep); check 02:46Z (26 Sep); packet 01:39Z, 02:38Z (28 Sep) |
| `in_bjp_elib_party_document_vol6_economic_resolutions` | 3,111,032 | `043834819a685a58cf6b9ae8eb2379e3ce47c1d76a93dd7713ae470374babb07` | check about 02:53Z (26 Sep), twice; packet 01:39Z, 02:38Z (28 Sep) |
| `in_rs_written_answers_19910108_usq1586_rath_yatra` | 100,324 | `df546af7447066924d4b2b83f2e82c0f74fb0a5061ab8e0649e90fb48725b9ad` | research 01:58Z, 02:22Z, 02:44Z (26 Sep); check 02:46Z (26 Sep); packet 01:40Z, 02:38Z (28 Sep) |
| `in_bjp_elib_party_document_vol3_presidential_speeches_part2` | 2,822,399 | `d12b2d2d86819f7ddebac2b8eb51096fbed3c931d47f3c4220342836ded59a20` | research 01:48Z, 02:16Z, 02:19Z (26 Sep); check 02:46Z (26 Sep); packet 01:40Z, 02:39Z (28 Sep) |
| `in_bjp_org_advani_profile_19961019` | 3,567 | `463e92e7a0ac792268fed6c6718d9e187b1385a2a823e5f0c12d21db3cf38ad0` | research 02:13Z, 02:44Z (26 Sep); check 02:47Z (26 Sep); packet 01:40Z, 02:39Z (28 Sep) |
| `in_bjp_org_lka_profile_biodata_20030317` | 10,760 | `41036bbbe496d2fa54de7e2eaefd1eede2c5b952b9248c9c778a82d3afd96955` | research 02:13Z, 02:44Z (26 Sep); check 02:47Z (26 Sep); packet 01:40Z, 02:39Z (28 Sep) |
| `in_bjp_elib_party_document_vol10_evolution_of_bjp` | 1,409,320 | `2d33ba5938c217d67d1cbfc715a1b6b7c3d6dcb88307d090177f61ad0d79f6a4` | research 01:49Z, 02:16Z, 02:19Z (26 Sep); check 02:46Z (26 Sep); packet 01:41Z, 02:39Z (28 Sep) |
| `in_rs_debate_19911210_ekta_yatra_bjp_president` | 83,271 | `16d162004276da02b0ad7259d74ef8ce3cd7c210aa3787a2b6a12a2bb6a4bf23` | research 01:59Z, 02:22Z, 02:44Z (26 Sep); check 02:46Z (26 Sep); packet 01:41Z, 02:39Z (28 Sep) |
| `in_rs_written_answers_19920304_usq1060_rocket_fire` | 44,815 | `5c3a7881bdd28d660196c83357ff6b169badac4d3d0890092307ea6a816936e4` | research 02:03Z, 02:22Z, 02:44Z (26 Sep); check 02:46Z (26 Sep); packet 01:41Z, 02:39Z (28 Sep) |
| `in_rs_written_answers_19920304_usq1067_flag_hoisting` | 59,344 | `fc4df69aa85e1287999c6c70b43b7b16be3cb1a3465fe37492db7ec747fe807f` | check about 02:51Z (26 Sep), twice; packet 01:41Z, 02:39Z (28 Sep) |
| `in_bjp_elib_party_document_vol2_presidential_speeches_part1` | 2,118,173 | `d6b44cbfd05642a16cb1b57959a03a21b01f384d039ad3871410c1384ff6df83` | research 01:49Z, 02:16Z, 02:19Z (26 Sep); check 02:46Z (26 Sep); packet 01:41Z, 02:39Z (28 Sep) |
| `in_bjp_org_swarna_jayanti_press_release_19970716` | 24,191 | `0eb50f1109f6b90d928a72f52a3870304b946a2e18f5c797e8318fdf2681918e` | research 02:12Z, 02:44Z (26 Sep); check 02:47Z (26 Sep); packet 01:43Z, 02:39Z (28 Sep) |
| `in_bjp_elib_party_document_vol4_foreign_policy_resolutions` | 2,538,135 | `ba547ac20a962cc9bd983d1064126abecdd37577da5d20c95c12c3bfaa77729b` | check about 02:53Z (26 Sep), twice; packet 01:43Z, 02:39Z (28 Sep) |
| `in_bjp_site_advani_opening_remarks_ne_19980411` | 35,726 | `76916f4cc8e0f55f9d75025c5c125e89efafcf813a002e4c0e710da6fcf31904` | research 02:07Z, 03:48Z (26 Sep); check 03:50Z, 04:28Z (26 Sep); packet 01:43Z, 02:39Z (28 Sep) |
| `in_bjp_site_thakre_presidential_speech_gandhinagar_199805` | 35,968 | `c4bc6c891a41fc6d49b2a7ab94e2c75cec2d30235d888cd50a8c6cf3e935f8c2` | research 02:13Z, 03:49Z (26 Sep); check 03:50Z, 04:28Z (26 Sep); packet 01:44Z, 02:40Z (28 Sep) |
| `in_bjp_site_thakre_budget_statement_19990227` | 8,112 | `22db19ee147876c4e31af13920e9a9513a659b8380b771eec12daa77ac0fc04e` | check 04:09Z, 04:43Z (26 Sep); packet 01:44Z, 02:40Z (28 Sep) |
| `in_bjp_site_thakre_press_statement_20000704` | 4,440 | `7c5119c79ec709e9c37f248f95dc058cf14e0f5edaadd9ddf57352e3a89359da` | research 02:18Z, 03:39Z (26 Sep); check 03:51Z, 04:28Z (26 Sep); packet 01:44Z, 02:41Z (28 Sep) |
| `in_bjp_site_laxman_nomination_20000802` | 10,983 | `5394657a29dd4cd0984bc5614afa0ded540f80258032e6a897d41295ca7b508c` | research 02:18Z, 03:40Z (26 Sep); check 03:51Z, 04:28Z (26 Sep); packet 01:44Z, 02:41Z (28 Sep) |
| `in_bjp_site_laxman_presidential_address_nagpur_200008` | 133,649 | `6672c98e7612b6f292d5d1873721d166553af43e02e2406c165592a1defbb5cb` | research 02:16Z, 03:40Z (26 Sep); check 03:51Z, 04:28Z (26 Sep); packet 01:44Z, 02:42Z (28 Sep) |
| `in_bjp_site_general_secretary_report_nagpur_200008` | 44,599 | `5471202185e3fb71b2398bb5864a82039fba56f5a5051f9554284c73d5a18c60` | check 04:04Z, 04:43Z (26 Sep); packet 01:45Z, 02:42Z (28 Sep) |
| `in_bjp_site_laxman_statement_20000831` | 18,262 | `9554a703e65331364b736c4335b42b6d28fcdb7c568647e7ffc5bd35b54f643e` | check 04:04Z, 04:42Z (26 Sep); packet 01:45Z, 02:43Z (28 Sep) |
| `in_bjp_site_laxman_statement_20000901` | 20,465 | `6c94660b81ac31c8e6b5f652e9def50232360868663b8dd22d05641d86a99b33` | research 02:18Z, 03:40Z (26 Sep); check 03:52Z, 04:29Z (26 Sep); packet 01:45Z, 02:43Z (28 Sep) |
| `in_bjp_site_laxman_press_statement_20010309` | 7,888 | `0808ef1f2f34083bcc059c706c506533787e035bd28038c1926f441dcef4d7c4` | research 02:20Z, 03:40Z (26 Sep); check 03:52Z, 04:30Z (26 Sep); packet 01:46Z, 02:43Z (28 Sep) |
| `in_bjp_site_office_bearers_press_release_20010314` | 5,816 | `b9261b7f54969d294a8aae7f75a7377adc2f9f06a72ffb30a4ff39c9f148fe5b` | research 02:16Z, 03:41Z (26 Sep); check 03:53Z, 04:30Z (26 Sep); packet 01:46Z, 02:43Z (28 Sep) |
| `in_bjp_site_press_release_20010315` | 11,219 | `e8ac6e05e2a64247533c1cba2f22af2766bd8a7ca31cea12043c4dd06646ebaa` | research 02:16Z, 03:41Z (26 Sep); check 03:53Z, 04:32Z (26 Sep); packet 01:46Z, 02:43Z (28 Sep) |
| `in_bjp_site_jana_krishnamurthy_press_statement_20010318` | 10,853 | `15a82af6b69e49cd56f33b52592d2e1d357e87d598fb4a5510f421f29bbf063c` | research 02:21Z, 03:41Z (26 Sep); check 03:53Z, 04:32Z (26 Sep); packet 01:46Z, 02:43Z (28 Sep) |
| `in_bjp_site_jana_krishnamurthy_ne_address_20010324` | 23,578 | `1acdbd7ba5bf2a1cad69e15a0ddd455f8f20d7e464aacfc4c319eef7f58a5bc3` | research 02:21Z, 03:41Z (26 Sep); check 03:53Z, 04:32Z (26 Sep); packet 01:46Z, 02:43Z (28 Sep) |
| `in_bjp_site_ne_resolution_political_situation_20010324` | 16,071 | `2d45d7b043094324437189fb65c1867f8fc946b50f644e94fd8a34c0f7cbf01a` | research 02:23Z, 03:42Z (26 Sep); check 03:54Z, 04:32Z (26 Sep); packet 01:47Z, 02:43Z (28 Sep) |
| `in_bjp_site_jana_krishnamurthi_press_statement_20010329` | 16,372 | `1b9e9a4e98f3fed99028c7aa9e203c3a6abee6e669146ae66c5a253a8c59c482` | research 02:21Z, 03:45Z (26 Sep); check 03:55Z, 04:32Z (26 Sep); packet 01:52Z, 02:44Z (28 Sep) |
| `in_bjp_site_shastri_press_statement_20020624` | 5,870 | `94bfc48904da4d8023372709a9b53e131c2962fb3ec65027ca517fd0dd2f8037` | research 02:16Z, 03:45Z (26 Sep); check 03:55Z, 04:33Z (26 Sep); packet 01:52Z, 02:44Z (28 Sep) |
| `in_bjp_site_naidu_profile_20020701` | 22,063 | `b524e4d3ee878f82f3c2699982f2438880ee7cd754059e03184b58b8f6e18b92` | research 02:16Z, 03:46Z (26 Sep); check 03:57Z, 04:33Z (26 Sep); packet 01:53Z, 02:44Z (28 Sep) |
| `in_bjp_site_naidu_first_press_conference_20020711` | 17,606 | `e379da92f8bff0219ac6f529fb184fd8bb2e8c531a8ac4be126264c75a566dde` | research 02:22Z, 03:46Z (26 Sep); check 03:58Z, 04:33Z (26 Sep); packet 01:53Z, 02:44Z (28 Sep) |
| `in_bjp_today_editorial_20020716` | 8,628 | `121728d4ebf9617d8fb7382979a9472a8ceb00c103630b5e0f637de4aa0fd499` | check 03:59Z, 04:42Z (26 Sep); packet 01:53Z, 02:45Z (28 Sep) |
| `in_bjp_site_naidu_national_council_address_20020803` | 86,460 | `b864c13744e4a6707d14eb4fae549967a57f5d3846fda9de8521ea4b194a79ca` | check 03:59Z, 04:43Z (26 Sep); packet 01:54Z, 02:45Z (28 Sep) |
| `in_bjp_site_press_statement_naidu_resignation_20041018` | 5,685 | `fe4232cb0a40f0852fbf7aa92a995185497426a1a538ead01cb737f2edb4743b` | research 02:09Z, 03:46Z (26 Sep); check 03:59Z, 04:33Z (26 Sep); packet 01:54Z, 02:45Z (28 Sep) |
| `in_bjp_site_advani_statement_20041020` | 25,103 | `1129703f17c57ddc5ab0df95e16843ee1c96e9ddc48c05a4838037aa41313208` | research 02:10Z, 03:46Z (26 Sep); check 03:59Z, 04:34Z (26 Sep); packet 01:54Z, 02:46Z (28 Sep) |
| `in_bjp_today_national_council_report_20041027` | 13,669 | `a92ec6f429b0e59b73cb334c0c67517bbf0ff9f71128342f8532dad00f933776` | check 03:56Z, 04:42Z (26 Sep); packet 01:54Z, 02:46Z (28 Sep) |
| `in_bjp_site_press_release_office_bearers_20041030` | 79,065 | `e48ed67a018b75fdd7ab78ea31d7bcea158edb1dac6290a06979ab0909637cd0` | research 02:10Z, 03:47Z (26 Sep); check 03:59Z, 04:34Z (26 Sep); packet 01:55Z, 02:46Z (28 Sep) |
| `in_bjp_site_resolution_advani_resignation_20050608` | 7,570 | `8dff1d2cd5a2afac46a0e914df6d35449309fc13c4f0e46323b6fcac24fbfa29` | research 02:11Z, 03:47Z (26 Sep); check 03:59Z, 04:34Z (26 Sep); packet 01:55Z, 02:46Z (28 Sep) |
| `in_bjp_site_advani_speech_20050615` | 24,489 | `356702ffa3d27c80a685290fa1f99ed69a8d6b739e36953f85c3d3f64e7c2c09` | research 02:24Z, 03:47Z (26 Sep); check 04:00Z, 04:34Z (26 Sep); packet 01:55Z, 02:47Z (28 Sep) |
| `in_bjp_site_advani_opening_remarks_ne_20051226` | 13,330 | `b6f4e69e65b89f13240e43631ceb39c72c058e0792937280f7cc5b9e20bbe304` | research 02:12Z, 03:47Z (26 Sep); check 04:00Z, 04:34Z (26 Sep); packet 01:55Z, 02:47Z (28 Sep) |
| `in_bjp_site_rajat_jayanti_sandesh_20051230` | 40,023 | `6099088f1d715f4fb3152ee96d1d0161a0f98bbf6c1494afbc424cac6c13f316` | check 04:10Z, 04:43Z (26 Sep); packet 01:56Z, 02:47Z (28 Sep) |
| `in_bjp_site_rajnath_singh_statement_20060102` | 8,405 | `01d1b4061f41be0f17163f428cae2f7a4de71e6d262dba7cab10ba646b3553bf` | research 02:12Z, 03:48Z (26 Sep); check 04:01Z, 04:35Z (26 Sep); packet 01:56Z, 02:47Z (28 Sep) |
| `in_bjp_site_rajnath_singh_national_council_address_20060120` | 70,234 | `c0940aca25290fd58f8fcfec8dd634c6d03b77692dce88a0781281dd06c2be51` | research 02:13Z, 03:48Z (26 Sep); check 04:01Z, 04:35Z (26 Sep); packet 01:58Z, 02:47Z (28 Sep) |
| `in_bjp_site_profile_gadkari_national_president_20091218` | 42,195 | `512d377daefe9acffa8a6587f1e2d8dcb0a0e1afca7be18e5f3e7a97e59dc568` | research 03:45Z, 04:24Z (26 Sep); check 23:41Z (27 Sep); packet 01:58Z, 02:47Z (28 Sep) |
| `in_bjp_site_parliamentary_board_resolution_rajnath_singh_20091219` | 27,136 | `96a0f6d7df42d380f0ecd28ce63406c5b0fb580457eec732af9f067191749ec1` | research 03:44Z, 04:24Z (26 Sep); check 23:41Z (27 Sep); packet 01:58Z, 02:48Z (28 Sep) |
| `in_bjp_site_gadkari_first_press_conference_as_president_20091224` | 34,928 | `7e18342428d0969d7b935798fa834858022abf7b0dfa485012824f48b3d773e9` | research 03:48Z, 04:25Z (26 Sep); check 23:42Z (27 Sep); packet 01:58Z, 02:48Z (28 Sep) |
| `in_bjp_site_press_release_listing_20091228` | 47,742 | `a8499b4367337d9b36ece18ab8f0acb696d1b702b1c14fb1d7a441fb3c0a9956` | check 00:37Z, 01:08Z (28 Sep); packet 01:59Z, 02:48Z (28 Sep) |
| `in_bjp_site_gadkari_presidential_address_national_council_indore_20100218` | 80,095 | `aa6b085313f78b276cb61ca1a9101af1e8e12b4a892fa6200839ea2130580867` | research 04:05Z, 04:44Z (26 Sep); check 23:42Z (27 Sep); packet 01:59Z, 02:50Z (28 Sep) |
| `in_bjp_site_gadkari_announces_national_executive_20100316` | 83,228 | `af9c1cf809bf9e3642bbda3941f879a571845711639b0edbb8d6dbf3e0e09256` | research 03:43Z, 04:23Z (26 Sep); check 23:44Z (27 Sep); packet 01:59Z, 02:50Z (28 Sep) |
| `in_bjp_site_gadkari_statement_no_second_term_20130122` | 82,918 | `6bd1f49a8a29213e5c6ca62331d7e360b7cc4736afb6d046abc81358c48f6782` | research 04:08Z, 04:44Z (26 Sep); check 23:45Z (27 Sep); packet 02:01Z, 02:50Z (28 Sep) |
| `in_bjp_site_national_election_officer_declaration_rajnath_singh_20130123` | 510,935 | `32e7d314a50dfc1b7abd93810ebfda57f7d596121c131bc22da45b8cf92df532` | research 04:13Z, 04:48Z (26 Sep); check 23:45Z (27 Sep); packet 02:01Z, 02:50Z (28 Sep) |
| `in_bjp_site_rajnath_singh_acceptance_speech_newly_elected_20130123` | 81,330 | `e9a82d0f54c95d802d4c77b28f93815ff97ee2b42813cc9ec54001092557914d` | research 04:10Z, 04:46Z (26 Sep); check 23:46Z (27 Sep); packet 02:01Z, 02:51Z (28 Sep) |
| `in_bjp_site_gadkari_styled_former_president_20130127` | 84,724 | `1f00274dcc0a43150f1bbd84e4834c2900d6ed9d4aa479abe2c8934f5026b12c` | research 04:09Z, 04:45Z (26 Sep); check 23:46Z (27 Sep); packet 02:01Z, 02:51Z (28 Sep) |
| `in_bjp_site_rajnath_singh_presidential_address_national_council_20130302` | 159,181 | `9d046824395207416f23a1d6e0dc0579d6a0657e2b1a8346fc953c8b785c49c7` | research 04:11Z, 04:47Z (26 Sep); check 23:46Z (27 Sep); packet 02:02Z, 02:51Z (28 Sep) |
| `in_bjp_site_bjp_presidents_1980_2013_list_20140117` | 60,215 | `ccbb3f8afa3129fb2033480b687556250ca04c041f5108978ba90e0eb15086f3` | research 03:41Z, 04:22Z (26 Sep); check 23:47Z (27 Sep); packet 02:02Z, 02:51Z (28 Sep) |
| `in_bjp_site_parliamentary_board_resolution_rajnath_singh_20140709` | 69,735 | `918e060f286920e258676dff45bd8ec8f1ecc3569a475e0a621e4262094ce115` | research 03:38Z, 04:15Z (26 Sep); check 23:47Z (27 Sep); packet 02:02Z, 02:52Z (28 Sep) |
| `in_bjp_site_profile_national_president_amit_shah_20140709` | 74,948 | `561aec7e5ee5064417d238009b5549bfda3a7b02e5e86fbf4de3cece64a7084e` | research 03:39Z, 04:16Z (26 Sep); check 23:48Z (27 Sep); packet 02:05Z, 02:52Z (28 Sep) |
| `in_bjp_site_amit_shah_presidential_address_national_council_20140809` | 101,609 | `c886ac72423db1a97e791d787d82072cf55d6285f8d8c763dc67885ea610d1da` | research 03:40Z, 04:21Z (26 Sep); check 23:48Z (27 Sep); packet 02:05Z, 02:52Z (28 Sep) |
| `in_bjp_site_amit_shah_re_elected_national_president_20160124` | 72,410 | `805c932deaa508f32577bd2f88eee416ee73134e1e6ada0e396407b7fb0f33ec` | research 02:52Z, 04:14Z (26 Sep); check 23:49Z (27 Sep); packet 02:23Z, 02:53Z (28 Sep) |
| `in_bjp_site_amit_shah_felicitation_20160202` | 76,078 | `d8104fcf252978ce78732b4833f8e462d080b4cd5fcde3505dd71a46ce0e8f86` | check 00:36Z, 01:07Z (28 Sep); packet 02:17Z, 02:53Z (28 Sep) |
| `in_ks_amit_shah_valmiki_jayanti_address_20161015` | 18,829 | `68a8cd1917e184da0be0e1789062f9d031b69b6ae8e63005a56c993546e27c89` | check 00:04Z, 00:43Z (28 Sep); packet 02:17Z, 02:53Z (28 Sep) |
| `in_ks_nadda_appointed_working_president_20190617` | 30,947 | `3003480b831e8a1bfeea844409a09de8c0f11242a95ec4b33dcbcec24c4025ea` | check 00:02Z, 00:41Z (28 Sep); packet 02:17Z, 02:53Z (28 Sep) |
| `in_kamal_sandesh_vol15_no03_20200201` | 3,452,018 | `03d5e769396cc0f7509238f2b3e16921f81ebd0df8fd63be3c30e1c41cd74c79` | check 00:47Z, 01:18Z (28 Sep); packet 02:17Z, 02:54Z (28 Sep) |
| `in_bjp_site_nadda_letter_to_karyakartas_20230117` | 109,764 | `8bdac60556ae1e4b6b0bd8982fa4ae9f9a22c4366f9a7f9a2e9d86bd8d8030f9` | check 00:37Z, 01:09Z (28 Sep); packet 02:17Z, 02:54Z (28 Sep) |
| `in_kamal_sandesh_vol18_no03_20230201` | 3,893,800 | `86b4773c6eb826ddd38ce3668824f99a55c44849d2a61852be5513e4fdf5251a` | check 00:49Z, 01:26Z (28 Sep); packet 02:18Z, 02:54Z (28 Sep) |
| `in_ks_nadda_address_national_convention_20240217` | 24,183 | `255b81f465be62d2f6870dbc70cbeafe83b1838671276397aca5bdc052ceddd6` | check 00:03Z, 00:42Z (28 Sep); packet 02:18Z, 02:54Z (28 Sep) |
| `in_bjp_site_bjp_presidents_list_20251120` | 83,494 | `523a4c17c0a46fa41f27b5402adfef127a81ce76e8f370fa3621210bd0ed5978` | research 03:35Z, 04:06Z (26 Sep); check 23:49Z (27 Sep); packet 02:18Z, 02:54Z (28 Sep) |
| `in_kamal_sandesh_vol20_no24_20251216` | 1,855,901 | `4436fe02345c3cfa06f9d605f6e74bf282ee58adc71ead469325af5de66f80ad` | check 00:48Z, 01:25Z (28 Sep); packet 02:22Z, 02:54Z (28 Sep) |
| `in_bjp_site_returning_officer_notice_20260116` | 659,783 | `24f5fc1b928e4ed6109c1eaa5ae37b1bd4cea6ba8e3cdc4bfc7972426cb11557` | check 00:26Z, 00:57Z (28 Sep); packet 02:22Z, 02:54Z (28 Sep) |
| `in_bjp_site_returning_officer_statement_20260119` | 441,845 | `43a4cdb66469889ed9bdcbfff4befce64c8ed6fc6dd07fd810a34ef983ef264c` | check 00:26Z, 00:57Z (28 Sep); packet 02:22Z, 02:54Z (28 Sep) |
| `in_bjp_site_nadda_felicitation_speech_20260120` | 101,761 | `b0daf4fca5ef0dd95843e0308970e42e9dc469b823a510ab440c68be2778eb6c` | check 00:23Z, 00:54Z (28 Sep); packet 02:22Z, 02:55Z (28 Sep) |
| `in_kamal_sandesh_vol21_no02_20260116` | 1,853,966 | `37da7adcbe88a98e051bd01181d0e27d5627e7b4501b6aed44f51d6ea2c56c8a` | check 00:47Z, 01:18Z (28 Sep); packet 02:23Z, 02:55Z (28 Sep) |
| `in_ks_national_president_appoints_office_bearers_20260817` | 87,779 | `9f0ee4234af85c678e14ecfc7894cb37103f602172fd6938b96061d0dd76301b` | check 00:02Z, 00:41Z (28 Sep); packet 02:23Z, 02:55Z (28 Sep) |

## Leads not imported

- Rajya Sabha, 5 December 1991, 'Re. Proposed Ekta Yatra by the BJP President' (store file
  ID_161_05121991_01_p228-247_1.pdf, 224,461 bytes, SHA-256 `3fa2219560e44ae9...`): a member speaks of the yatra 'being
  undertaken by the President of the BJP' without naming him; the record of 10 December 1991 names Joshi.
- Rajya Sabha, written answer of 18 December 1991, Unstarred Question 2927 in Hindi (store file
  IQ_161_18121991_U2927_p177.pdf, 46,398 bytes, `e92411cf3711e373...`): the question speaks of the yatra of the BJP
  President, unnamed, and the answer does not style Joshi President (A-R4).
- Rajya Sabha, 8 and 9 December 1993 (store files ID_169_08121993_01_p326-370_1.pdf and ID_169_09121993_01_p262-310_1.pdf):
  Advani, Joshi and others as members and 'BJP leaders', with no party office in the English text; the Hindi and
  Urdu-script pages of 9 December were not all reviewed. Rajya Sabha, 13 March 2001 (ID_192_13032001_01_p311-314_1.pdf):
  no mention of the party presidency.
- The party's 1996 profile of Dr. Murli Manohar Joshi (bjp.org/bjp/profiles/joshi.html, 2,632 bytes,
  `d7601fd79fe2fc42...`), which stops at his posts as General Secretary; the 1998 leadership index (leader/index.html:
  undated campaign material captured on 19 May 1998) and Advani profile (leader/advaniji.htm, a copy of the 1996
  sentence); K. R. Malkani's undated 'BJP History' (history/history.html, capture of 9 February 1999).
- Vol-3's addresses of 1986-1990 in the section headed 'SHRI LK ADVANI' (library.bjp.org item 248): they name no speaker,
  and those of 1986 and 1988 are before the period. Vol-10's undated sentence that Joshi, 'who had just taken charge as
  party president', decided on the Ekta Yatra. Vol-2's address of 21-23 August 1998 (PDF p.287) recalling the taking of
  charge at the Gandhinagar session: a recollection with no day. The e-Library's manifestos of 1991, 1996 and 1998 and
  'Policy Document Vol 4', which do not state the presidency.
- bjp.org news/nr2a13.htm (capture of 21 February 1999, the National Executive resolution of 12 April 1998 thanking 'the
  party President, Shri L.K. Advani'): the same resolution is recorded from Vol-4 (A-R2).
- bjp.org today/offmem.html (captures of 7 December 1998 and 3 February 1999): an undated office-bearer list headed
  'President Shri Kushabhau Thakre'. The news indexes news.htm (captures of 28 January 1999 and 3 September 2000): finding
  aids whose 1999 statements add only continuations after 27 February 1999.
- bjp.org pages not needed or not usable: 'Press on 672000.htm' (6 July 2000, a continuation), Thakre-Speech.htm (10
  November 1998, no styling), leader/kusha.htm and leader/Bangaru-bio.htm (stale profiles), leader/JanaBio-data.htm (an
  undated bio-data), news/Feb2802.htm (dated 'February 28, 2001' in a page of 2002), 'June 2701.htm', 'June 2402a.htm'
  and 'July 2102.htm' (continuations), 'July 0102a.htm' (a later copy of the recorded Naidu profile),
  Press/Office_Bearers_1.htm (redundant with the release of 30 October 2004), Press/Sep_2005/sep_1605.htm and
  dec_2805.htm (continuations with nothing on the transition), jan_0206_p.htm (Rajnath Singh's profile, which stops in
  2004) and office-bearers_jan_2506.htm (a continuation).
- BJP Today, November 1-15, 2004, Advani's address to the National Council (today/Nov_0104/Nov_1_p_12.htm, 41,455 bytes,
  `f0ee949d694d0733...`): recollections of Naidu's resignation and its acceptance, redundant with the release of 18
  October 2004 (B-R3).
- BJP Today's contents pages for August 16-31, 2000 (today/Aug-2000b1.htm, 9,578 bytes, `c82c864632458c81...`) and
  January 16-31, 2006 (today/jan_0206/Contents_jan_0206.htm, 7,148 bytes, `266b43d17667b8c5...`): finding aids whose
  articles (Laxman as President-elect; Rajnath Singh's interview, profile and presidential address) are not archived
  (B-R9, B-R10).
- Newspaper reprints and clippings on bjp.org (nr1a02.htm, a Tribune report that the National Council would meet from 1
  to 3 May 1998 to elect the President; nr2a07.htm, a report in The Hindu of the April 1998 nomination schedule;
  news/profile.htm; the clippings linked from the home pages of 2 August 2002 and 21 October 2004): news, leads only.
- bjp.org pages of 2009-2016: content/view/3096 (a leadership page of 18 December 2009 with '2009 - present'),
  id=8509 (the release page of 23 January 2013 that only links the declaration PDF, which is recorded),
  national-council-meeting-at-jawaharlal-nehru-stadium (7 August 2014) and national-president-s-election-at-11-ashoka-road
  (20 January 2016), both without body text, press-bjp-national-president-shri-amit-shah (August 2014, a continuation, not
  fetched), the Hindi PDF of Rajnath Singh's National Council address of March 2013 (Presidential Speech Sh Rajnath Singh
  ... March 03, 2013_hindi.pdf; the English page is recorded), and the home pages and listings used as finding aids.
- Kamal Sandesh web posts 10341 and 10343 (the Prime Minister's and Nadda's speeches of 20 January 2020), corroborative
  only; post 40433 (10 June 2026), a continuation before the 17 August 2026 record; the live WordPress API, including its
  `_fields` and search queries, which is rendered per request and is never recorded.
- Kamal Sandesh post 41236 (kamalsandesh.org/wp-json/wp/v2/posts/41236, on the National President's meeting with
  ambassadors on 1 September 2026): the only rendering found was modified on 17 September 2026, after the cutoff, and
  the Internet Archive holds no capture of it up to 6 September 2026 (check C14).
- Pre-cutoff captures of the Kamal Sandesh API objects of posts 10345, 10337, 10347, 25282, 38371 and 38671: superseded
  by the stored issues that print the same reports (C-R16 to C-R18, C-R20, C-R22 and C-R23).
- The party's releases captured at 20260122033851 (Nabin's felicitation speech of 20 January 2026, 110,230 bytes,
  `e0ce59727cf9285b...`), 20260126175154 (Republic Day, 26 January 2026, 92,842 bytes, `81822b1975f91a07...`),
  20251217103812 (the Working President's first day, 16 December 2025, in Hindi, 94,168 bytes, `919c21a13e37c6f6...`)
  and 20251225171922 (Nadda at a Christmas celebration, 25 December 2025, 95,596 bytes, `3897a5a2f5d28853...`): an
  acceptance, continuations and a working-presidency claim, redundant with the recorded records (C-R4, C-R5, C-R10,
  C-R11). Their page template carries a rotating block that is not the release's.
- The BJP Central Library's Kozhikode National Council resolutions of September 2016 and 2017 newsletters
  (handle/123456789/276 and handle/123456789/468): further continuations for Amit Shah, not downloaded.
- The party's leader profiles as captured by Arquivo.pt on 20 November 2025 (shri-nitin-gadkari, shri-rajnath-singh-1,
  shri-amit-shah, shri-jagat-prakash-nadda): retrospective and undated for the presidency; the /bjp-presidents list is
  recorded instead.
- The wikipedia articles on the party's presidents and newspaper reports of the elections: leads only, not consulted as
  evidence.

## Sources attempted

- www.bjp.org (the live party site): no HTTP response to curl on 26 and 27 September 2026 (TLS connects through
  Akamai, the server asks for renegotiation and then sends nothing). Between 02:30Z and 02:45Z on 26 September 2026 the
  Part C research also tried it with a browser User-Agent and Accept headers, which the house rules count as trying to
  pass bot filtering; those attempts got no response, their wording is removed from the records (check C16), and nothing
  was retried. Every bjp.org record here is a pre-cutoff capture.
- The Parliament Digital Library (eparlib.sansad.in): connection timeouts, so the Lok Sabha debates could not be
  searched; the sansad.in bio-data paths for Advani, Joshi and Gadkari answered 500. The Library of Congress web archive
  (webarchive.loc.gov) answered with a Cloudflare challenge, which was not attempted. www.eci.gov.in answered 406 to curl.
- The web archive: CDX queries for www.bjp.org/former-presidents found no captures; its captures of www.bjp.org around
  17 June 2019 and 20 January 2020 are 403 responses; the Parliamentary Board statement of 10 June 2005 (june_1005a.htm)
  and Arun Jaitley's statement of 19 September 2005 were never captured; the root-level kusha.htm,
  leader/July+0102a.htm and news/Mar2501.htm answered 404 or an empty body; the release page that links the Returning
  Officer's statement answered 404 although listed (the PDF it links is recorded); and KS_ENG_Feb%202024_2.pdf (capture
  20240421031228) is a truncated 1,048,576-byte record that PDFium cannot open. The Rajya Sabha data service does not
  filter by date, so the 8 January 1991 sheet was located by its neighbouring question number.
- web.archive.org answered 429, 500, 503 or 504 or refused connections at times between 26 and 28 September 2026;
  requests were retried with backoff, and no recorded identity is an error page.

No site terms, licences or cookie banners were accepted, no CAPTCHA or challenge was attempted, and no login was used.

## Checker defects

| # | Defect | Outcome |
|---|---|---|
| A1 | The Vol-3 section dividers were called blank | **Applied**: the visual review, the farewell's uncertainty and the lead note record the named dividers (pp.1, 73 and 203) and that the farewell is filed under 'DR. MM JOSHI' at p.28 and 'SHRI LK ADVANI' at p.85; one rule for Vol-2 and Vol-3: an address whose own text names no speaker has holder_name null |
| A2 | Vol-2 addresses took their speaker from the section divider | **Applied**: the 1993, 1995 and 1998 addresses have holder_name null and never date a holder; Advani's 1997 observation rests on the party's named release |
| A3 | `attested_period` on thirteen claims | **Applied**: removed; multi-day meeting headings and retrospective spans store no structured date and keep the printed words; single-day headings keep their day |
| A4 | The part's own event kinds | **Applied**: one vocabulary for the three parts; `in_office_attestation` only for holder observations; the test pins the kinds and bans the old ones |
| A5 | Nine printed variants in holder_name | **Applied**: holder_name is the normalised name or null; printed forms stay in the texts |
| A6 | Holders cited claims of mixed kinds and days | **Applied**: every observation cites only in-office attestations of its own day, and the test pins it |
| A7 | Row keys | **Applied**: `observation_id` is `in_eci_20240323_np_03`, `review_observation` holds BJP-PRES-xx, `role_title` is added, in the INC key order |
| A8 | The 2 May 1998 handover claim collapsed three events | **Applied**: separate claims for the handover announcement, Thakre's unanimous election, the President-elect salutation and the farewell at p.316; 'lay down' is quoted in its own tense |
| A9 | The 18 Jun 1993 nomination and stewardship were one claim | **Applied**: a `nomination_proposal_stated` claim and a `predecessor_reference` claim |
| A10 | The Calcutta address of 10 Apr 1993 says nothing about the office | **Resolved by removal**: the claim is dropped |
| A11 | The April 1990 resolution narrates 10 Dec 1989 | **Applied**: `narrated_styling` with no structured date; the Madras resolution of 21-23 Jul 1990 (Vol-6, A-R1) is the in-window party record |
| A12 | Vol-5 p.136 was omitted | **Applied**: `in_bjp_ne_resolution_party_president_advani_hawala_199602`, no structured date, the title and the surname quoted as printed |
| A13 | The Jaipur meeting headings were not used | **Applied**: the Vol-4, Vol-5 and Vol-6 headings are cited as context in the farewell's uncertainty; no start or end |
| A14 | The 8 Jan 1991 page prints no date | **Applied**: the locator, uncertainty and scope say the date is the Secretariat's, and the store file of the same sheet that prints '[ 8 JAN. 1991 ]' is recorded as the extract's date anchor with its own identity |
| A15 | 'In this period' is ambiguous | **Applied**: the ambiguity is stated and no structured date is stored |
| A16 | The history's 1997 styling carried the yatra's date | **Applied**: kept at 1997-05-18 as a `retrospective_biography_statement` dated by the day it recalls; its date is pinned as never a holder date |
| A17 | The 9-10 Nov 1990 claim did not say that it prints only the title and the surname | **Applied**: said in the text |
| B1 | Thakre's 'I assume this office' was missing | **Applied**: `in_bjp_thakre_states_he_assumes_office_199805`, `assumption_stated_undated`, no structured date and no `from` |
| B2 | `attested_period` on five claims | **Applied**: removed; attested_on null; the session dates stay in the texts |
| B3 | Naidu and Advani had both attested_on and a proposed start | **Applied**: Naidu has `from` 2002-07-01 and no attested_on; Advani's start is declined and he is observed on 20 Oct 2004; every holder has the model's shape |
| B4 | Advani's proposed start cited the appointment | **Applied**: the appointment is an `appointment_decision` claim; the recollection is `assumption_recalled_retrospective`; no start (see B3) |
| B5 | Naidu's profile was misquoted and used as a start | **Applied**: 'Ist July 2002 onwards President' is quoted as printed, as a `retrospective_term_span`; the start rests on BJP Today's editorial (B-R1) |
| B6 | Jana Krishnamurthi's substantive presidency needed a ruling | **Applied**: the ruling is stated under How a start and an end are decided: no source states that the acting service of 14 Mar 2001 ended or became a presidency, so his service, including the 'National President' heading and the entrustment he recalled on 24 Mar 2001, is claims only; the test pins the acting window from 14 Mar 2001 to 30 Jun 2002 |
| B7 | The 24 Mar 2001 address's recollection held three events | **Applied**: the letter of 13 Mar 2001 (`resignation_tendered`), the recalled acceptance of the 14th and the recalled acting responsibility (no structured date) are separate claims; the letter's quoted text is noted |
| B8 | Same-day stylings merged with undated recollections (2001, 2006) | **Applied**: each styling is its own claim dated by the statement; the recollections store no structured date; the acceptance of 2 Jan 2006 stays dated and is not a start |
| B9 | The National Council's 2004 endorsement was not found | **Applied**: BJP Today's report (B-R2) is imported with separate claims for the endorsement, Naidu's farewell and Vajpayee's 'the 5th time'; the release's 'National Executive' claim is kept as printed and the conflict noted |
| B10 | Laxman's earlier styling was missed | **Applied**: observed on 31 Aug 2000 (B-R4); the 1 Sep 2000 statement is a continuation, and both takeover sentences are recollections |
| B11 | Thakre's observation was more than two years late | **Applied**: observed on 27 Feb 1999 (B-R6); the statement of 4 Jul 2000 is a continuation |
| B12 | Row shape and vocabulary | **Applied**: as A4 and A7 |
| B13 | The encoding notes misread the redirect's content type | **Resolved by removal**: the encoding notes are gone; every identity is the exact `id_` URL, which answers 200 with no redirect and no Content-Encoding |
| B14 | The Gandhinagar speech page carries another item's `<title>` | **Applied**: the source's scope note records the wrong title and that the page heading identifies the item |
| B15 | The 11 Apr 1998 remarks had the meeting's day as `published_date` | **Applied**: `published_date` null; the day of the remarks stays in the claims |
| B16 | Pages the researcher fetched were not listed | **Applied**: the newspaper reprints are leads only; BJP Today's contents pages of August 2000 and January 2006 are finding aids whose articles are unarchived (B-R9, B-R10); the Rajat Jayanti Sandesh is imported for the convention's dates (B-R8) |
| C1 | Gadkari's 2009 holder cited the profile and a special kind | **Applied**: it cites only `in_bjp_gadkari_first_press_conference_as_president_20091224`, an `in_office_attestation` |
| C2 | The profile's two dates | **Applied**: both printed dates recorded; `profile_heading_styling` with no structured date; the listing (C-R14) is imported as a `press_release_index_entry`; 'earliest styling' is gone |
| C3 | Gadkari's 2013 holder rested on the decision not to seek a second term | **Applied**: a separate `in_office_attestation` for the page's title and heading; the decision is its own claim |
| C4 | Rajnath Singh's 2013 claim: kind, day and quotation | **Applied**: `in_office_attestation`; the uncertainty says 2 Mar is the page's date and cites the Hindi file name; 'aftertaking' is quoted as printed in a separate recollection; the 23 Jan alternative is gone |
| C5 | Amit Shah's 2016 holder rested on the re-election | **Applied**: the heading is an `election_result` claim; observed on 2 Feb 2016 (C-R13) |
| C6 | Amit Shah's 2019 holder: a continuation claim and a handover end | **Applied**: it cites only the board-meeting styling; until null; both handover claims are `handover_statement`, never an until |
| C7 | Nadda 2020: both dates, and a recollection among the start claims | **Applied**: `from` 2020-01-20 only, citing only the taking-of-charge claim (`assumption_statement`); the election, declaration, nomination, life sketch and 2023 recollection are separate claims |
| C8 | Nadda's 2023 holder rested on the extension | **Applied**: observed from the party's page of 17 Jan 2023 (C-R12); the extension is a `term_extension` claim; the 2024 styling is a continuation |
| C9 | Nadda's end rested on a handover | **Applied**: until null; `handover_statement`; the note says the day is the successor's start; the release of 25 Dec 2025 is a lead (C-R11) |
| C10 | Nabin 2026: both dates, and a declaration among the claims | **Applied**: `from` 2026-01-20 only, from the party's same-day release (C-R3) and the party organ (C-R6); the declaration is a claim |
| C11 | Nabin's 17 Aug 2026 claim: kind and identity | **Applied**: `in_office_attestation`; the source is the Internet Archive capture of 4 Sep 2026 (C-R15), not the live rendering |
| C12 | Holder shape | **Applied**: every holder has the model's keys in the model's order |
| C13 | In-office attestations that fed no holder | **Applied**: the 2016 and 2024 stylings are `in_office_continuation_attestation`; the 1 Sep 2026 claim is dropped (C14) |
| C14 | The ambassadors' post is post-cutoff | **Resolved by removal**: the source and its claim are dropped; the post is a lead |
| C15 | 'The body changes only if the post is edited' | **Applied**: the wording is gone; the live WordPress API responses are replaced by pre-cutoff captures (three API objects and one page) or by the issues stored on www.bjp.org; no `_fields` URL is recorded |
| C16 | Browser User-Agent wording | **Applied**: every identity was fetched with curl's default User-Agent and `Accept-Encoding: identity`; the wording is removed; www.bjp.org is recorded as not answering and was not retried with other headers; the Part C research's attempt is disclosed under Sources attempted |
| C17 | Kamal Sandesh 'issue dated' titles and published dates | **Applied**: web posts are titled as web posts with their capture date; the stored issues give volume, number, fortnight and publication date; no `published_date` precedes the reported event |
| C18 | Lists and spans carried capture dates as attested_on | **Applied**: no structured date; `retrospective_term_span`; 'no structured date' in each uncertainty |
| C19 | The 2014 list's locators were off by one | **Applied**: the tenth, eleventh and twelfth entries |
| C20 | 'President: Shri. Nitin Gadkari' was not as printed | **Applied**: the heading and the entry are quoted separately |
| C21 | The 2013 acceptance heading was misquoted | **Applied**: quoted as printed; `president_elect_styling` |
| C22 | `attested_period` on three claims | **Applied**: removed; no structured date |
| C23 | The 2020 handover claim took another article's day | **Applied**: no structured date; `handover_statement` |
| C24 | The 2026 nomination claim collapsed four events | **Applied**: separate nomination, scrutiny and sole-candidate claims from the Returning Officer's statement (C-R1), and the schedule from his notice (C-R2) |
| C25 | Three former Presidents in one nameless row | **Applied**: one `predecessor_reference` row each for Rajnath Singh, Amit Shah and Nitin Gadkari; Nadda's 'outgoing' styling is kept in the handover claim and in the party's release |
| C26 | Hindi sources quoted only in romanised form | **Applied**: the Devanagari names are quoted with translations; holder_name 'Amit Shah' |
| C27 | Two holder_name values for Nadda | **Applied**: 'J. P. Nadda' in every row and holder |
| C28 | Row shape and multi-valued review observations | **Applied**: rows converted; each claim has one review observation |
| C29 | Vocabulary | **Applied**: as A4 |
| C30 | The Returning Officer's records | **Applied**: both imported (C-R1, C-R2); the summary and the unresolved note are reworded |
| C31 | A source and a claim shared one id | **Applied**: the claim is `in_ks_nabin_assumes_working_presidency_20251215` and the web post is replaced by the stored issue (C-R8); the test asserts that no id is both a source and a claim |
| C32 | Kamal Sandesh's publisher | **Applied**: the scope notes state the basis (published on behalf of Dr. Mookerjee Smruti Nyas, linked from the party website's navigation and stored on www.bjp.org), and the stored issues are preferred |

Missing primary records the checks confirmed:

| # | Record | Outcome |
|---|---|---|
| A-R1 | Vol-6, Economic Resolutions (e-Library item 265) | **Imported**: `in_bjp_elib_party_document_vol6_economic_resolutions` (the Madras resolution of 21-23 Jul 1990) |
| A-R2 | Vol-4, Foreign Policy and Other Resolutions (item 266) | **Imported**: `in_bjp_elib_party_document_vol4_foreign_policy_resolutions` (the resolution of 11-12 Apr 1998) |
| A-R3 | Rajya Sabha, 4 Mar 1992, Unstarred Question 1067 (Hindi) | **Imported**: `in_rs_written_answers_19920304_usq1067_flag_hoisting` |
| A-R4 | Rajya Sabha, 18 Dec 1991, Unstarred Question 2927 (Hindi) | **Declined**: the President is unnamed and Joshi is already observed on 10 Dec 1991; a lead |
| B-R1 | BJP Today, July 16-31, 2002, 'Letter from the Editor' | **Imported**: `in_bjp_today_editorial_20020716` (Naidu's start) |
| B-R2 | BJP Today, November 1-15, 2004, National Council report | **Imported**: `in_bjp_today_national_council_report_20041027` |
| B-R3 | BJP Today, November 1-15, 2004, Advani's address | **Declined**: recollections redundant with the release of 18 Oct 2004; a lead |
| B-R4 | Laxman's statement of 31 Aug 2000 | **Imported**: `in_bjp_site_laxman_statement_20000831` (Laxman's observation) |
| B-R5 | General Secretary's report, National Council, Nagpur, August 2000 | **Imported**: `in_bjp_site_general_secretary_report_nagpur_200008` |
| B-R6 | Thakre's budget statement of 27 Feb 1999 | **Imported**: `in_bjp_site_thakre_budget_statement_19990227` (Thakre's observation) |
| B-R7 | Presidential address to the National Council, 3 Aug 2002 | **Imported**: `in_bjp_site_naidu_national_council_address_20020803` |
| B-R8 | Rajat Jayanti Sandesh, 30 Dec 2005 | **Imported**: `in_bjp_site_rajat_jayanti_sandesh_20051230` (context only) |
| B-R9 | BJP Today, August 16-31, 2000, contents | **Declined**: a finding aid whose article is not archived; a lead |
| B-R10 | BJP Today, January 16-31, 2006, contents | **Declined**: a finding aid whose articles are not archived; a lead |
| C-R1 | The National Returning Officer's statement, 19 Jan 2026 | **Imported**: `in_bjp_site_returning_officer_statement_20260119` |
| C-R2 | The National Returning Officer's notice, 16 Jan 2026 | **Imported**: `in_bjp_site_returning_officer_notice_20260116` |
| C-R3 | Nadda's speech at the felicitation, 20 Jan 2026 | **Imported**: `in_bjp_site_nadda_felicitation_speech_20260120` (Nabin's start) |
| C-R4 | Nabin's speech at the felicitation (captured 22 Jan 2026) | **Declined**: an acceptance and a predecessor reference redundant with the same-day records; a lead |
| C-R5 | Republic Day release, 26 Jan 2026 | **Declined**: a continuation after the stated start; a lead |
| C-R6 | Kamal Sandesh, Vol. 21, No. 02 (stored issue) | **Imported**: `in_kamal_sandesh_vol21_no02_20260116` |
| C-R7 | Kamal Sandesh, Vol. 15, No. 03 (stored issue) | **Imported**: `in_kamal_sandesh_vol15_no03_20200201` |
| C-R8 | Kamal Sandesh, Vol. 20, No. 24 (stored issue) | **Imported**: `in_kamal_sandesh_vol20_no24_20251216` |
| C-R9 | Kamal Sandesh, Vol. 18, No. 03 (stored issue) | **Imported**: `in_kamal_sandesh_vol18_no03_20230201` |
| C-R10 | The Working President's first day, 16 Dec 2025 (Hindi) | **Declined**: a working-presidency claim redundant with the party organ's report; a lead |
| C-R11 | Nadda at a Christmas celebration, 25 Dec 2025 | **Declined**: a continuation after the observation of 15 Dec 2025; a lead |
| C-R12 | Nadda's letter to the party workers, 17 Jan 2023 | **Imported**: `in_bjp_site_nadda_letter_to_karyakartas_20230117` (Nadda's 2023 observation) |
| C-R13 | The felicitation release of 2 Feb 2016 | **Imported**: `in_bjp_site_amit_shah_felicitation_20160202` (Amit Shah's 2016 observation) |
| C-R14 | The Press Releases listing captured on 28 Dec 2009 | **Imported**: `in_bjp_site_press_release_listing_20091228` (the profile's second date) |
| C-R15 | Kamal Sandesh page captured on 4 Sep 2026 | **Imported**: the recorded URL of `in_ks_national_president_appoints_office_bearers_20260817` |
| C-R16 | Capture of Kamal Sandesh API post 10345 | **Declined**: superseded by the stored issue Vol. 15, No. 03 (C-R7), which prints the same report |
| C-R17 | Capture of Kamal Sandesh API post 10337 | **Declined**: as C-R16 |
| C-R18 | Capture of Kamal Sandesh API post 10347 (life sketch) | **Declined**: as C-R16 |
| C-R19 | Capture of Kamal Sandesh API post 7435 | **Imported**: the recorded URL of `in_ks_nadda_appointed_working_president_20190617` |
| C-R20 | Capture of Kamal Sandesh API post 25282 | **Declined**: superseded by the stored issue Vol. 18, No. 03 (C-R9) |
| C-R21 | Capture of Kamal Sandesh API post 30863 | **Imported**: the recorded URL of `in_ks_nadda_address_national_convention_20240217` |
| C-R22 | Capture of Kamal Sandesh API post 38371 | **Declined**: superseded by the stored issue Vol. 20, No. 24 (C-R8) |
| C-R23 | Capture of Kamal Sandesh API post 38671 | **Declined**: superseded by the stored issue Vol. 21, No. 02 (C-R6) |
| C-R24 | Capture of Kamal Sandesh API post 13 | **Imported**: the recorded URL of `in_ks_amit_shah_valmiki_jayanti_address_20161015` |

## Suggested next work orders

These are proposals for the integrator. They are not created in `work-orders.json`.

- `C01-India-BJP-002`: the party's records of the elections or selections of 1991, 1993, 1995, 1998, 2000, 2006, 2009,
  2013 and 2014 (declarations, National Council ratifications and the days of taking charge), from the party's print
  archive (BJP Today, Kamal Sandesh) or the e-Library.
- `C01-India-BJP-003`: the Lok Sabha debates of 1990-2026 from a network that can reach the Parliament Digital Library,
  and the live bjp.org former-president pages if the site answers.
- `C01-India-BJP-004`: any extension of J. P. Nadda's tenure after June 2024 and any amendment of the party constitution
  on the term.
- `C01-India-PARTY-001`: the leaders of the other national parties in the ECI table (outside this packet).

## Integration notes (outside this packet's file boundary)

- **Base and claim:** the claim commit `27c52dc4` holds only the handoff and was made on `2ee3b146` (CLAUDE-C01-20, since
  integrated). The branch merged current integration at `da49f9c0` (`a33a8987`), `26a07bb8` (`01d5217c`) and `29f1937e`
  (`e41aa18d`, `codex/campaign-certification` at the latest fetch); those merges changed `research-index.json` (other
  packets' totals) but not `india.json`, the India tests or `campaign_research.py`. This packet only appends to
  `india.json`: 74 sources after the CLAUDE-C01-20 sources; on
  `in_eci_20240323_np_03` one role, its sources and claim ids and one coverage note; one packet coverage note after the
  CLAUDE-C01-20 note.
- No existing extract is edited: the 74 extracts under `research/sources/` are new files, and no CLAUDE-C01-11,
  CLAUDE-C01-15 or CLAUDE-C01-20 source, claim, holder or extract changes.
- `research-index.json` is the only file this packet shares with other pending packets. It is regenerated in a
  **separate commit**. New totals against the base: 1,403 sources and 3,898 claims (previously 1,329 and 3,731);
  organization and institution observations are unchanged (841 and 34). India now has 4 role observations (previously
  3), 558 claims (previously 391), 84 entries pending mapping and the same nine discovery batches. The index must be
  regenerated after any other pending packet merges.
- Pinned tests updated, none loosened and no assertion removed:
  - `test_india_research_s10e.py`: counts (entries, sources, claims, roles) (84, 275, 558, 4), from (84, 201, 391, 3);
    the organization roles are pinned to exactly `in_eci_20240323_np_03` (`in_bjp_president`) and `in_eci_20240323_np_05`
    (`in_inc_president`), and the test that recognition grants no role is re-expressed for those two observations (every
    other observation still has none, and all keep their empty mapping, lifecycle and status); the CLAUDE-C01-20 checks
    now select that packet's role by id, and the 74 new sources are pinned to four hosts (the web archive,
    library.bjp.org, the Rajya Sabha store and Arquivo.pt), the 2026-09-28 access date and their rows to the party role;
    the source list is the original, C01-11, C01-15, C01-20 and then C01-27 sources.
  - `test_india_prime_ministers_c01_11.py` and `test_india_presidents_c01_15.py`: the source list now ends with the BJP
    role's sources (74 sources and 167 claims pinned); roles 4 (from 3); each packet's own claims share none with the
    BJP role, and in `test_india_prime_ministers_c01_11.py` the rows beyond C01-11's are the presidency's or a party
    role's, the BJP rows numbered BJP-PRES only; the packet coverage has eleven notes with CLAUDE-C01-20's at index 9
    and CLAUDE-C01-27's last; the index has four role observations.
  - `test_india_inc_presidents_c01_20.py`: the party-leader roles are exactly the BJP role and the INC role; the source
    list ends with the BJP sources; roles 4 (from 3); the packet coverage has eleven notes with the INC note at index 9
    and the BJP note last; the index has four role observations. Its INC assertions are unchanged.
- `test_campaign_census` runs in this sparse worktree (its `spheres-sim/data` is in the sparse checkout); results are in
  the handoff. The sparse checkout was not widened.
- The atlas (`tools/ui/leadership-research-review.js`) shows "Observed on" for a holder with `attested_on`; Bangaru
  Laxman's note carries his end. No UI code changed.
- The party role's event-kind vocabulary is pinned in `test_india_bjp_presidents_c01_27.py`: it shares 21 of its 60
  kinds with the CLAUDE-C01-20 vocabulary, and the other 39 are its own (among them kinds for National Council and
  National Executive statements, resignations, the acting presidency, working presidencies, term extensions and the
  Returning Officer's steps).
- `research/README.md`, the C01 README totals and `docs/planning/ai-workstreams.json` are left for the integrator; this
  handoff is self-proposed and not registered there.

## Checks

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
```

Results are recorded in the handoff.
