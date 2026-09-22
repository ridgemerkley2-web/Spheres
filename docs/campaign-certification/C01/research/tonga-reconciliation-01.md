# Tonga reconciliation 01 — PTOA, the People's Party and Assembly selection

Packet: **CLAUDE-C01-01**. State: **ready_for_review** (not complete).
Owner: Claude. Integrator/reviewer: Codex. Branch `claude/c01-tonga-01`, base
`3d422da` on `codex/campaign-certification`. Research access: 21 September 2026
(local; IPU responses carry 22 September UTC). The historical cutoff stays
**7 September 2026**.

This packet reconciles ten Tonga observations in [tonga.json](tonga.json). It adds
three sources and seven claims. It adds no organization, institution, game mapping,
leader term, lifespan, portrait or avatar. The parent scope (C01, C06, S23, WC1 and CP1)
remains open.

## Outcome

| ID | Question | Decision |
|---|---|---|
| TO-REC-01 | Court of Appeal prints PATOA (para 21) and PTOA (para 32) | **Accepted:** PATOA is a variant printing of PTOA |
| TO-REC-02 | What organization does PTOA name? | **Accepted as a provisional grouping** with `to_dpfi`; legal identity unverified |
| TO-REC-03 | Tu'i'onetoa's prior affiliation | **Accepted** as a dated self-description; joining and leaving dates unknown |
| TO-REC-04 | PATOA/PTOA vs People's Democratic Party (`to_pdp`) | **Rejected** |
| TO-REC-05 | IPU 2010's ambiguous "Democratic party" label | **Unresolved**, still uncounted |
| TO-REC-06 | People's Party names | **Accepted** as name observations of `to_peoples_party`; tertiary acronyms rejected |
| TO-REC-07 | People's Party legal form | **Accepted:** incorporated society per April 2022 testimony; no registration date |
| TO-REC-08 | Is "Leader" the same office as society "President"? | **Unresolved**; kept as separate roles |
| TO-REC-09 | People's Party at the 2021 election | **Accepted:** Tu'i'onetoa elected under the TPP banner |
| TO-REC-10 | 2021 Assembly selection vs royal appointment | Assembly selection **accepted**; royal appointment **unresolved**, no holder added |

## Observations

### TO-REC-01 — PATOA is a variant printing of PTOA

Evidence: Court of Appeal AC 8 of 2022 (`to_court_peoples_party_20220809`). Paragraph 21
reads "PATOA" and paragraph 32 reads "PTOA". The Supreme Court judgment under
appeal, CV 75 of 2021 (`to_sc_kiu_v_tuionetoa_20220429`, para 324), quotes the
translated speech transcript (Exhibit 1(b)). The transcript reads **PTOA**. All three
passages describe the same 1 October 2021 Niutao speech.

Both court PDFs are JBIG2 scans with an OCR text layer. The labels were therefore checked
against rendered page images (appeal PDF pages 9 and 12; trial PDF pages 33–34), not OCR
text alone. The discrepancy is printed; it is not an OCR artifact.

Record: `to_dpfi.name_observations` keeps PATOA with `variant_of: "PTOA"`. The
appellate claim `to_tuionetoa_prior_party_speech_20211001` and its checked-in extract
are **unchanged**. They still record that the source's own labels disagree.

### TO-REC-02 — PTOA grouped with the Democratic Party of the Friendly Islands

Evidence: IPU's 2021 election record (`to_ipu_2021_ptoa_expansion`) names "the Democratic
Party of the Friendly Islands (PTOA)" as one of two political groups under whose banners
candidates ran. IPU 2010 (`to_dpfi_pohiva_2010`) names DPFI with 'Akilisi Pohiva as
leader. IPU 2017 (`to_ipu_2017_dpfi_continuity`) says the 14 elected independents who
supported Pohiva belonged to DPFI, "led by the Prime Minister", and that DPFI took eight
seats in 2014. IPU 2021 calls Siaosi Pohiva, son of the late Prime Minister 'Akilisi
Pohiva, "one of its leaders".

Decision: the IPU observations are grouped under `to_dpfi` as a
`provisional_observation_grouping`. The same English name and the Pohiva leadership
links connect them, so the grouping does not rest on the name alone. It is **not** a
legal identity, statute or registration finding; `automatic_merge` is false.

Limits: IPU says Tonga has no party system and that candidates formally stood as
independents. No founding statute, registered name or incorporation record for PTOA
was found. "One of its leaders" is not an office title and is not imported as a
holder observation.

### TO-REC-03 — Tu'i'onetoa's prior PTOA affiliation

Evidence: the transcript quoted in para 324 records Tu'i'onetoa saying that he started
with PTOA, that it changed course, and that he left to start a new vision. The Court of
Appeal (paras 21 and 32) describes the same passage as leaving "the opposition" party.

Decision: accepted as a speaker's self-description, attested on 1 October 2021. It
supplies **no** joining date, departure date, membership term or legal split. The
2019 PMO appointment remains a national-office observation and implies no party office.

### TO-REC-04 — PATOA/PTOA is not the People's Democratic Party

`to_pdp` is IPU's April 2005 split from FIHRDM under Tesina Fuko. None of the new
sources connects PTOA, PATOA or Tu'i'onetoa to PDP. The only overlap is that both names
contain "Democratic", which is not enough to match them. **Rejected.** `to_pdp` is unchanged.

### TO-REC-05 — IPU 2010's ambiguous "Democratic party" label

The 2021–2022 evidence concerns the 2021 election and says nothing about the separate
2010 label (`to_2010_ambiguous_democratic`). It is neither identified with PTOA/DPFI nor
counted. **Unresolved.** A 2010 primary candidate list is needed.

### TO-REC-06 — People's Party names

Accepted name observations on `to_peoples_party`, each tied to Tu'i'onetoa:

| Name as printed | Date | Source |
|---|---|---|
| Tonga People's Party | 28 May 2021 | IDCPC dialogue (existing) |
| People's Party | 1 Oct 2021 | Court of Appeal (existing) |
| Paati 'a e Kakai; the People's Party | 1 Oct 2021 | Supreme Court CV 75/2021, para 8 |
| Tonga People's Party (TPP) | 18 Nov 2021 | IPU 2021 |

The grouping stays `provisional_observation_grouping`. Tertiary summaries also use
other acronyms (PAK, PAKT) and a 2019 founding date. **Rejected for import:** no primary
source in this packet uses them.

### TO-REC-07 — People's Party legal form

Evidence: CV 75/2021 paras 33–35. A Ministry for Trade and Economic Development officer
whose work includes registering incorporated societies testified that the People's Party
is an incorporated society. She said it has 35 members, a constitution and stated aims,
with Tu'i'onetoa as president and Tevita Lavemaau as secretary. The trial ran 19–21 April 2022.

Decision: accepted as testimony attested within that trial window. The judgment gives no
registration date, society number, registered name or register extract. The lifecycle
therefore stays unknown. Incorporation as a society is not a statutory party registration.

The Court of Appeal allowed the appeal and set aside the trial court's declarations and
costs order (AC 8 of 2022, para 44). This packet uses that judgment **only** for
background identity evidence that the appeal did not concern. It imports no allegation,
finding, declaration or penalty.

### TO-REC-08 — Leader vs society President

Observed titles:
- "Leader of Tonga People's Party": IDCPC, 28 May 2021.
- "leader of the Paati 'a e Kakai": a descriptive use by the trial court for 1 October 2021.
- "president" of the incorporated society: testimony, April 2022.

Whether Leader and society President are one office depends on the society's
constitution, which was not obtained. **Unresolved.** The packet keeps
`to_peoples_party_leader` (`party_leader`, one IDCPC holder observation) separate from
two new `other` roles: society President (Tu'i'onetoa) and society Secretary (Tevita
Lavemaau). Both carry `attested_period` 19–21 April 2022, `from`/`until` null and no
term. The trial court's descriptive "leader" is recorded as a name observation, not as a
second leadership holder.

### TO-REC-09 — People's Party at the 2021 election

IPU 2021 states that Prime Minister Pohiva Tu'i'onetoa was elected under the TPP banner
at the 18 November 2021 election. Accepted as an electoral-banner observation; it is not a
party-office appointment.

### TO-REC-10 — 2021 Assembly selection kept apart from royal appointment

IPU 2021 states that on 15 December 2021 the new Assembly "elected" Siaosi Sovaleni as
the new Prime Minister. The same page describes the interim Speaker's process as electing
a **Prime Minister Designate**. Under Clause 50A (`to_constitution_pm_cabinet`), the King
appoints the Assembly-recommended member.

Decision: the Assembly selection is recorded on `to_pm` as a claim, **not** as a
holder observation. This follows the 2019 pattern: Assembly recommendation on 27 September,
royal appointment on 8 October.

Unresolved: the royal warrant date and effective date for 2021. A Matangi Tonga report
(28 December 2021) describes a Royal Warrant presented that morning, with office held
"from 27 Dec. 2021". That would make three separate dates. It is a secondary news
account and was **not imported**. `pmo.gov.to` refused connections throughout this
review, so the PMO primary release could not be retrieved.

The 2019 and 2025 separations already in the packet were left as recorded. The PMO
PDFs could not be re-fetched for this packet.

## Sources added

| Source ID | What | Retained provenance |
|---|---|---|
| `to_sc_kiu_v_tuionetoa_20220429` | Supreme Court of Tonga, Kiu v Tu'i'onetoa, CV 75 of 2021, 29 Apr 2022 ([AGO PDF](https://ago.gov.to/cms/judgements/supreme-court-civil/category/206-cv-2022.html?download=2386%3Akelekolio-kiu-v-pohiva-tuionetoa-cv-75-21-tosc-29-apr-2022-cooper-j-judgement)) | Downloaded 602,620 bytes, SHA-256 `57fb2ac3…3012372`; PDF pages 1, 2, 4, 33, 34 rendered and read |
| `to_ipu_2021` | [IPU Parline, Tonga 2021 election](https://data.ipu.org/parliament/TO/TO-LC01/election/TO-LC01-E20211118/) | Browser fetch, 162,654 bytes; **no hash asserted** (two fetches differed) |
| `to_ipu_2017` | [IPU Parline, Tonga 2017 election](https://data.ipu.org/parliament/TO/TO-LC01/election/TO-LC01-E20171116/) | Browser fetch, 166,618 bytes; no hash asserted |

Each has a checked-in derived factual extract under [sources/](sources/). The extract's
own checksum is in the packet and is distinct from any original-response hash. Original
bodies are not checked in. Direct non-browser requests to IPU returned HTTP 403.

Re-verification of existing evidence: the Court of Appeal PDF was re-downloaded and
reproduces the recorded 307,617 bytes and SHA-256 `ff8af757…dee90973`. Its pages 9 and
12 were re-read from rendered images.

## Suggested next work orders

These are proposals for the integrator. They are not created in `work-orders.json`.

- `C01-Tonga-REC-002`: prime-minister transition ledger for 2019, 2021, 2024 and 2025.
  Assembly selection, royal warrant and effective date come from PMO releases or the
  Gazette, once `pmo.gov.to` is reachable.
- `C01-Tonga-REC-003`: People's Party incorporated-society register entry: society
  number, registration date, registered name, constitution and officer rules. This
  resolves TO-REC-07 and TO-REC-08.
- `C01-Tonga-REC-004`: DPFI/PTOA founding statute and September 2010 launch; the
  registered Tongan name; leadership after 'Akilisi Pohiva, whom IPU 2021 calls "the late
  Prime Minister"; the 2025 result.
- `C01-Tonga-REC-005`: 2010 candidate list to settle TO-REC-05, plus the
  HRDM/HRDMT/FIHRDM naming chain.

## Integration notes (outside this packet's file boundary)

- `research-index.json` is regenerated in a **separate commit** on this branch. Codex
  can take it as is or rerun `python -X utf8 tools/avatars/campaign_research.py`. New
  totals: 61 sources and 1,613 claims (previously 58 and 1,606). Organization,
  institution, packet and batch counts are unchanged. Tonga still has one open batch,
  `C01-Tonga-DISC-B001`, with nine members.
- `research/README.md` ("Together the nine packets … 1,606 claims across 58 cited
  sources") and the S10.g paragraph in `C01/README.md` need the new totals and a pointer
  to this report. They are left for the integrator.
- `test_tonga_research_s10g.py` pins exact counts and was updated: 16→19 sources,
  28→35 claims, and exactly one `party_leader` role with the new name observations
  appended. No assertion was removed or loosened.

## Checks

```text
python -X utf8 tools/avatars/campaign_research.py --check
python -X utf8 -m unittest discover -s tools/avatars -p "test_tonga_*.py"
python -X utf8 -m unittest discover -s tools/avatars -p "test_campaign*.py"
node --test tools/ui/check_leadership_research_review.cjs
python tools/planning/workboard.py --session C01
git diff --check
```
