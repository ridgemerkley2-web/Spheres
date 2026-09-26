# Opening mandate audit: six concentrated A1 countries

Read-only, 22 September 2026. No historical data, vote shares, simulation flags, thresholds or gameplay were changed. The accompanying JSON records source access limits, current code fields and hashes. This is a six-case review, not complete opening-roster certification.

The code currently initializes every government with `elected=false` and `unrestricted_mandate=false`. The public-mandate helper therefore returns zero before the first simulated ballot, although a six-month party performance record can already activate political-confidence losses. No simulated election does not mean no previous public mandate.

| Country | Table date and office match | Assessment for a separate prior-ballot record |
|---|---|---|
| Suriname | 25 November 1987; Front/Shankar matches `sr_fdo`. | Prior competitive mandate is documented. Retained military powers remain material. |
| Guatemala | 3 November 1985; Cerezo/DCG matches `gt_dcg`. | Prior electoral mandate is documented, but neither all-party freedom nor the model's synthetic DCG+UCN coalition is historically certified. |
| Guyana | 9 December 1985; Hoyte/PNC matches `gy_pnc`. | Published result is disputed; do not translate its declared share into an unrestricted popular mandate. |
| Comoros | Future March 1990 result; opening president is interim, and the leader table refuses a party tie. | No January 1990 ballot mandate can be imported from this result. |
| São Tomé and Príncipe | Future 20 January 1991 result; opening incumbent is Pinto da Costa/MLSTP. | Future PCD victory cannot confer consent on the opening one-party regime. |
| Myanmar | Future 27 May 1990 result; opening incumbent Saw Maung is Army-tied. | Future NLD vote cannot confer a civilian mandate on SLORC. |

## Suriname

The [National Assembly's own history](https://www.dna.sr/achtergrond-info/geschiedenis-dna/een-korte-geschiedenis/) dates free, secret general elections to 25 November 1987. The [IACHR's 1987–88 report](https://cidh.oas.org/annualrep/87.88eng/chap4e.htm) records the Front's 40 seats, Shankar's installation in January 1988 and the Army's constitutional position in national government. Popular electoral support and military autonomy coexisted; recognizing the former cannot erase the latter.

The exact 85.5% already in code was not independently reverified against a primary tally. The [official result PDF](https://verkiezingen.gov.sr/storage/pdf/verslagen/1987/uitslag-verkiezingen-25-november-1987-okb.pdf) was located but returned HTTP403. The cached 1990 CIA edition instead prints a coarse80%; that is not sufficient grounds to replace the current percentage. The game's four selected rows sum to0.981, so normalization yields about87.16% for `sr_fdo`, another reason to avoid labeling a live model number an exact historical percentage. The IACHR report contains an inconsistent November15 mention alongside November25; the parliament independently confirms25.

## Guatemala

[IPU's contemporary election report](https://data.ipu.org/election-summary/PDF/GUATEMALA_1985_E.PDF), ChronicleXX pp79–81, confirms the3November1985 ballot, DCG38.7% and51of100seats, and Cerezo's inauguration on14January1986. The current table's38.64% is a finer national-list transcription; the game then recalculates seats and forms DCG+UCN. The reviewed primary report does not substantiate UCN as a member of the inherited administration. A dated prior mandate should identify the actual Cerezo/DCG administration, not certify the whole synthesized coalition or substitute the distinct presidential-runoff share.

The [IACHR's 1987–88 Guatemala report](https://cidh.oas.org/annualrep/87.88eng/chap4b.htm) recognizes constitutional civilian government while recording ongoing violence and the PGT's illegal status. Prior competitive electoral legitimacy is therefore not proof that all political organizations were unrestricted. Do not rewrite a modern game freedom flag from the ballot date alone.

## Guyana

[IPU's 1985 report](https://data.ipu.org/election-summary/PDF/GUYANA_1985_E.PDF), ChronicleXX pp83–84, confirms Hoyte/PNC and the declared78.5%/42directlyelectedseats, alongside opposition allegations of widespread fraud and the PPP's withdrawal. The source establishes a declared result and office continuity, not a reliable numerical measurement of freely expressed consent. Date and party matching alone would be insufficient eligibility tests here.

## Comoros, São Tomé and Myanmar

Comoros's March1990 table postdates the start; the game's leader record explicitly refuses to backdate Djohar's later party affiliation. A [UN primary report indexed online](https://digitallibrary.un.org/nanna/record/105269/files/E_CN.4_1991_14-EN.pdf?registerDownload=1&version=1&withMetadata=0&withWatermark=0), E/CN.4/1991/14 paragraphs70–73, distinguishes his December1989 acting presidency from the11March1990 election. Direct PDF access returned403, so the receipt labels this indexed text, not a fully inspected document.

[São Tomé's national parliament history](https://www2.camara.leg.br/saotomeeprincipe/parlamento/historial) and [IPU's 1991 report](https://data.ipu.org/election-summary/HTML/2275_91.htm) place the multiparty constitutional reform in1990 and the first competitive parliamentary ballot inJanuary1991. The code's normalized valid-party shares and IPU's percentages of all ballots use different denominators; neither belongs toJanuary1990.

The verified cached [CIA1990 text](https://www.gutenberg.org/cache/epub/14/pg14-images.html) records Myanmar's dissolved legislature, suspended constitution, military incumbent and still-futureMay1990 ballot. Its general information date and March political-update cutoff are distinct; election dates were checked explicitly. An [indexed UN report](https://digitallibrary.un.org/record/228428/files/E_CN.4_1996_65-EN.pdf) also dates the NLD result to27May1990, but its fullPDF was inaccessible. NoJanuarymandate depends on that inaccessible document.

## Bounded implementation implications

If the design proceeds, use a separate explicitly dated record of a prior competitive ballot and actual incumbent party, without setting `g.elected`, initializing a vote anchor, changing schedules, granting a honeymoon or awarding consolidation. Keep historical consent, contemporary model support, restrictions and civilian military control distinct. Match the actual office holder; exclude future results, interim offices, disputed claims and source gaps. Clear or invalidate an imported record on a real incumbent change, coup, opening transition or incompatible ban. Old saves lacking the record should not silently receive newly invented historical facts. A subsequent valid modeled election can replace the historical eligibility through its existing path.

This audit supports considering Suriname and Guatemala for such a carefully scoped record. It does not establish an unrestricted mandate for all six, a new exact vote percentage, or a passing A1 outcome.
