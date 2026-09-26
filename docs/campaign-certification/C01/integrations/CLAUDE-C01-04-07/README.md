# CLAUDE-C01-04 and CLAUDE-C01-07 — bounded intake review

**Decision: accepted as bounded research intake, with the existing uncertainty
and open questions retained.** Reviewed by Codex on 22 September 2026. No blocking
factual defect was found in the critical claims independently reopened below.
This accepts the research packets, not complete country coverage, production
leader eligibility, portraits, a campaign certification, or the parent C01 gate.

Reviewed submissions:

- CLAUDE-C01-04: `62690e61ebf32750ae0b392f8171ad314ec3ac43`, merged by `48d8bfef`.
- CLAUDE-C01-07: `e6f9fa4140ec881eec73fa8770e4b3bbf4700cac`, merged by `8d8e4b41`.
- The independent review read the merged packet at
  `8d8e4b412b12389a96e6fc6eefde967bca696188`, the two handoffs, and
  [DPFI report](../../research/tonga-dpfi-04.md) and
  [Crown report](../../research/tonga-crown-07.md). The historical cutoff remains
  7 September 2026; later retrieval dates do not extend the observed history.

## Independent source checks

[source-fetch.json](source-fetch.json) records fresh response identities and
locators. Twelve direct downloads from the official publishers or the specified
Internet Archive captures matched both the submitted byte count and SHA-256.
Source responses were hashed in memory. No original PDF, HTML response, source
photograph, seal or signature is checked into this integration record.

The two IPU pages returned HTTP 403 to the independent Python request. Their
published content was reopened through the web reader instead. This confirms the
statements below, but does **not** independently reproduce their response hashes
or the 2014 page's normalized hash. Those limitations remain explicit in the
evidence and no source identity was silently replaced.

| Critical question | Independently checked finding | Limit retained |
| --- | --- | --- |
| PTOA name and Helu's office | [Supreme Court CV 55/2022](https://ago.gov.to/cms/judgements/supreme-court-civil/category/206-cv-2022.html?download=2491:fatai-helu-and-paula-piveni-piukala-v-electoral-commission-the-election-of-lord-nuku-cv-55-22-tosc-29-aug-2022-whitten-qc-cj-ruling), page 1, paragraph 3 and footnote 3, visually checked: the PTOA/Democratic Party naming, Tongan expansion, and Fatai Helu's presidency appear in the ruling dated 29 August 2022. | A background recital, not an adjudication of party office. It supplies no election/appointment date, office term, successor, legal incorporation or proof that President equals party leader. The longer English DPFI equivalence still depends on the separately cited IPU record. |
| Pohiva in 2014 | [IPU 2014](https://data.ipu.org/election-summary/HTML/2317_14.htm), election background, names Pohiva as DPFI leader at the 27 November election. It separately dates the Assembly's selection to 29 December and Cabinet taking office to 19 January 2015. | An IPU election attestation, not a continuous party term or appointment instrument. The relative royal-endorsement wording is not promoted to a new exact appointment date. |
| 2025 results | [Electoral Commission results](https://elections.gov.to/wp-content/uploads/2025/11/Tonga-General-Elections-Results-revised-20251.pdf), pages 1–2 visually checked: Puloka 1,343, Siaosi Pohiva 758 and Fifita 45 in Tongatapu 1; Sika 951 and Fasi 764 in Tongatapu 2. Tables identify people and polling places, without a party column. [IPU 2025](https://data.ipu.org/parliament/TO/TO-LC01/election/TO-LC01-E20251120/) marks party results inapplicable. | Neither source establishes PTOA's party seat total. Refusing to infer zero seats is correct. Later petitions, final seat status and affiliations are not inferred. Only pages 1–2 were independently read in this review; the packet's structural review of pages 3–17 was not repeated. |
| Tupou IV's death and George Tupou V's succession | [Archived PMO reproduction of Gazette 20](https://web.archive.org/web/20070514085818id_/http://www.pmo.gov.to/artman/publish/article_170.shtml) gives the same death instant as 23:34 on 10 September in New Zealand and 00:34 on 11 September 2006 in Tonga. The independently re-downloaded English Gazette 19 reproduction states that the Crown devolved through the predecessor's death. | The Tongan calendar boundary is 11 September; the New Zealand date is retained separately. These are archived PMO reproductions, not located gazette facsimiles. Proclamation, funeral and mourning remain separate events. |
| George Tupou V's death and Tupou VI's accession | [Gazette Extraordinary 8](https://ago.gov.to/cms/images/LEGISLATION/GAZETTES/2012/2012-0008/GazetteExtraordinaryNo.8of2012.pdf), page 1 visually checked, dates the death to 18 March 2012 at 15:58 Hong Kong / 20:58 Tonga. [Gazette Extraordinary 9](https://ago.gov.to/cms/images/LEGISLATION/GAZETTES/2012/2012-0009/GazetteExtraordinaryNo.9of2012.pdf), page 1 visually checked, links devolution to that death and issues the proclamation on 19 March. | The 18 March accession boundary combines the explicit devolution statement with the death notice. It is not the proclamation date or a coronation date; no unsupported reign end is assigned to Tupou VI. |
| Monarch during 1990 | [Income Tax amendment](https://ago.gov.to/cms/images/LEGISLATION/AMENDING/1990/1990-0004/IncomeTaxAmendmentAct1990.pdf), page 1, and the Constitution amendment of 1990 name Tupou IV in dated assents on 12 July and 8 August. The [UN meeting record of 12 September 2006](https://documents.un.org/doc/undoc/gen/n06/520/19/pdf/n0652019.pdf), pages 1–3, supplies the longer reign context, including Tonga's representative's 1965 accession statement. | The Acts attest the holder during 1990; neither creates a July/August reign start. The outside-period accession remains year-level context, with the packet's structured start left null. |
| Regencies | The archived PMO wreath report of 13 March 2006 styles Tupouto'a Prince Regent; the 1 June throne speech identifies Princess Regent Pilolevu. [2006 Tonga Law Reports](https://ago.gov.to/cms/ago-materials/publications/tonga-law-reports.html?download=1560:2006_tlr), PDF pages 166, 168 and 170 visually checked, independently records the 1 June opening and its lawful exercise during the King's absence. | The court treats the appointment as given; it does not supply the missing instrument or term bounds. These remain Crown observations, not reigns or additional King holders. |

## Data and acceptance boundaries

The merged `tonga.json` keeps `to_dpfi_president` separate from
`to_dpfi_leader`. Helu and the additional Pohiva party observations retain null
term starts/ends. The King's holder list retains the supported death/devolution
boundaries, leaves Tupou IV's start and Tupou VI's end null, and adds no regent as
a monarch. These distinctions agree with the independently reviewed records.

This was a targeted independent source audit, not a claim that every new sentence
in the 22-claim and 44-claim packets was reverified. In particular, this review did
not independently repeat the 2017 contextual party identification, the complete
2019 vacancy/oath chain, the society-register search, or the 2008/2015 coronation
source chains. Their submitted locators and uncertainty remain available for
subsequent work. This review does not upgrade the 2015 public-body corroboration
to a ceremonial instrument.

The unresolved founding and legal form, post-2019 party leadership, missing
regency instruments and dates, coronation oaths, missing 2006 gazette facsimiles,
and requested 2015 ceremony record remain open. A search that found no record is
not proof that no record or organization exists. The rejected zero-seat inference
for 2025 stays rejected.

The integrator separately reran the 49 Tonga tests and research-index check;
those checks were reported passing before this source audit. They are structural
verification, not substitutes for reading the sources. No simulation, installed
leader, avatar, central workboard or research claim was changed by this review.
