# C04 preparation: France/Tonga fictional successor pilot

**Proposal-only · CLAUDE-C04-PREP-01 · prepared 27 September 2026 · base `e41aa18d`.**
Nothing in this folder is installed, granted, illustrated or scheduled.
The eight people below are **invented characters**. None is a real person or a forecast.
C04 is **not** complete: full C04 still requires accepted C02 inputs, reviewed artwork and a later integration decision.

| File | What it holds |
| --- | --- |
| [`proposals.json`](proposals.json) | The eight drafts, the sourced role catalog, the excluded (hereditary) roles and party rows, the Tonga name guard, the unresolved mappings and observations. |
| [`sources.json`](sources.json) | 28 sources with access dates, locators and uncertainty notes: 18 read (bytes and SHA-256 of each response) and 10 blocked, unreachable or missing. It also holds a 30-entry fetch log. |
| This review | A readable summary for reviewers. |

Validator: [`tools/avatars/check_successor_proposals.py`](../../../../../tools/avatars/check_successor_proposals.py).
Tests: [`tools/avatars/test_successor_proposals.py`](../../../../../tools/avatars/test_successor_proposals.py).

On 30 September 2026, the fourth Tonga draft was renamed from Kalolo Vaikona to **Kalolo Matalehu** because the expanded research corpus mentions the prior surname. The unchanged name guard rejects it. The replacement passes the current corpus checks; an exact-name lead search returned no results. This does not prove that no private person shares the name. Earlier search counts and original review receipts below remain historical records of the original packet.

## The eight drafts

Every window is a **potential eligibility window**, not a prediction. A draft can be seated only by an actual vacancy or election reached in play, and every incumbent is kept. Ages give the youngest possible age when the window opens and the oldest when it closes.

| Draft ID | Invented name | Role (category) | Party row | Born | Window | Ages |
| --- | --- | --- | --- | ---: | --- | --- |
| `draft_c04_fr_01` | Ondine Rivasseau | PS First Secretary (party leadership) | `fr_ps` | 1978 | 2029-09-08 → 2035-12-31 | 50–57 |
| `draft_c04_fr_02` | Loïc Ferrandou | PCF National Secretary (party leadership) | `fr_pcf` | 1983 | 2026-09-08 → 2035-12-31 | 42–52 |
| `draft_c04_fr_03` | Gwenola Pérignac | Presidential contender (executive eligibility) | `fr_ps` | 1974 | 2029-09-08 → 2035-12-31 | 54–61 |
| `draft_c04_fr_04` | Anatole Villaume | National Assembly deputy (collective institution) | `fr_fn` | 1988 | 2026-09-08 → 2035-12-31 | 37–47 |
| `draft_c04_to_01` | Lesieli Fotu | People's representative (collective institution) | none | 1984 | 2026-09-08 → 2035-12-31 | 41–51 |
| `draft_c04_to_02` | Sitani Lolohea | Prime Minister via a people's seat (executive eligibility) | none | 1971 | 2026-09-08 → 2035-12-31 | 54–64 |
| `draft_c04_to_03` | Pisila Tukuafu | Non-elected Cabinet Minister (collective institution) | none | 1979 | 2026-09-08 → 2035-12-31 | 46–56 |
| `draft_c04_to_04` | Kalolo Matalehu | PTOA leader (party leadership) | none | 1976 | 2026-09-08 → 2035-12-31 | 49–59 |

Each nation covers party leadership, executive eligibility and a collective institution. France has no hereditary office. Tonga's hereditary offices are excluded outright (see below).

The two PS windows conservatively open on 8 September 2029. A draft cannot have been a party member before the cutoff. These authored paths accrue three consecutive years of membership before national-body or candidacy eligibility (arts. 2.6.5 and 5.1.5). The statutes contain exceptions, but this pilot uses none. In particular, the explicit National Council exception for legislative, senatorial and European candidacies is not assumed to waive presidential eligibility. Any exception needs separate source and party-decision review.

## Rules applied to every draft

- **Fiction boundary.** Before 8 September 2026 a draft has only an undated private or professional background. There is no public office, party membership, candidacy or election. Every political step is a conditional gameplay possibility inside the window. Narrative text contains no calendar years.
- **No manufactured lineage.** Parentage is `not_specified`; no title, estate or royal or noble descent is claimed.
- **Succession contract.** `actual_succession_events_only`, incumbents retained, no date trigger, no predicted appointment, no automatic acting role. Every window has at least one vacancy or election gate, and no gate carries a date. This matches `policy()` in `spheres-sim/src/party_leadership_future.rs`.
- **Names.** Each name was checked in three ways:
  - The validator compared it with the repository's real-person corpus: 878 names from the census/roster, portrait and figure records, C01 research holders and the backlog.
  - It also checked the full text of the C01 research packets and every name token recorded in each nation's research record.
  - A lead web search on 27 September 2026 found no public figure with any final name. Ten earlier candidates were discarded because a search found an identical or near-identical real person, a surname shared with a sitting French senator, or a close association with a real pageant contestant. Private individuals cannot be excluded, so editorial review is still required.
- **Appearance.** Briefs are text only, in the approved full-body cartoon style. They carry no party logos, sashes, regalia or real-person likeness, and no image was generated.

## France

**Institutional distinctions (sourced).**
- The President is elected for five years by direct universal suffrage, for at most two consecutive terms (Constitution arts. 6-7).
- The President appoints the Prime Minister and, on the Prime Minister's proposal, the Government (art. 8). Government membership excludes a parliamentary mandate (art. 23).
- 577 deputies are elected for five years (arts. 24-25; Assemblée fiche n° 3). A deputy must be French, aged 18 and eligible to vote. Groups need fifteen members and name their president (Règlement art. 19).
- Presidential candidates must be 18 and eligible to vote, and need 500 presentations by elected officials from 30 or more departments or collectivities (ministry memento of 2022, citing Law 62-1292 art. 3).
- The republican form of government cannot be revised (art. 89), so there is no hereditary office.
- **PS:** the First Secretary is elected by all members from the first signatories of the two leading motions (art. 3.2.7). Members designate the presidential candidate (art. 5.3.1).
- **PCF:** the national secretary heads the list that wins at congress (art. 12.4). Congress meets at least every three years (art. 8), and executive functions are generally limited to nine years (art. 13).
- **RN:** its president is elected by the General Assembly on nominations from 20% of the enlarged National Council (art. 10), a separate office from any deputy or group president.

**Rows used and excluded.**
- `fr_ps` and `fr_pcf` have continuous names.
- `fr_fn` is recorded in the roster as the Front national renamed Rassemblement national in 2018.
- `fr_rpr` and `fr_udf` are **excluded**. Their roster identity notes forbid invented post-2002 RPR leaders and any invented single post-2007 UDF successor chain.

**Dossiers.**
- **`draft_c04_fr_01` Ondine Rivasseau.** An invented labour-law lecturer who ran a legal clinic. She could become First Secretary only through a congress vote or a National Council election after a prolonged vacancy, after three years of in-play membership. Her role is `party_only`: a first secretary is not a national executive.
- **`draft_c04_fr_02` Loïc Ferrandou.** An invented rail technician turned training instructor. He could become national secretary only if his list wins a congress convened in play. An authored gate requires one prior congress cycle. The draft is for the national-secretary title only; no party presidency is created.
- **`draft_c04_fr_03` Gwenola Pérignac.** An invented public-health physician. Her path is: PS designation by members, then 500 presentations, then winning a presidential election reached in play. She never becomes interim President. She would need a `presidential_contender` future-office grant, which does not yet exist (UM-02).
- **`draft_c04_fr_04` Anatole Villaume.** An invented pharmacist. He could become a deputy only by winning a legislative election or by-election in play, and a group president only if at least fifteen deputies choose him. The RN investiture procedure was not researched: the statutes contain none, and the investiture page lists members only.

## Tonga

**Why hereditary offices are excluded.** The following offices exist only through descent or a title:
- the Crown (clause 32, entrenched by clause 79);
- the Royal Family, the heir and a regency (clauses 27, 32 and 42-43);
- the nobles, whose titles descend by a fixed law of succession (clauses 44 and 111);
- the nine nobles' representatives, elected by and from the nobles (clauses 60 and 63);
- the Speaker, who must be a nobles' representative (clause 61).

A fictional holder of any of these would invent royal parentage or a peerage, which is forbidden. No elected-party succession path is invented for any of them.

The Privy Council is not hereditary, but it is called at the King's pleasure and decides title appeals (clause 50), so it is also excluded. The premiership can be reached from a nobles' seat: the Prime Minister's Office release of 18 December 2025 describes the appointee as a nobles' representative. The drafts use **only** the people's route.

Invented Tongan names are also screened against noble titles and royal names. These come from "Lord/Lady/Prince/Princess/HRH/HSH" styles in the C01 Tonga record, the Crown holders, and a curated guard drawn from the same release. The guard is deliberately conservative.

**Non-hereditary roles used (sourced).**
- Seventeen people's representatives (clauses 59-60). Electors and candidates must be 21 or older, not nobles, registered in the constituency, and free of unpaid court orders (clauses 64-65).
- Nomination needs Form 4 signed by 50 electors, a $400 non-refundable deposit and court clearances (Electoral Act s. 9).
- The Prime Minister is an elected representative recommended by the Assembly: nominations are seconded by two members, followed by a secret-ballot majority with eliminations (clause 50A and Schedule). No-confidence votes are limited (clause 50B).
- Up to four Cabinet Ministers may be non-elected. They vote in the Assembly except on no-confidence motions (clause 51).
- Parties have no statutory framework. The Assembly describes no party system, returns carry no party labels, and a society's own rules decide its officers (registry memorandum of July 2026).

**Dossiers.**
- **`draft_c04_to_01` Lesieli Fotu.** An invented science teacher and cooperative coordinator from an outer island of Vava'u. She holds a people's seat only if she wins one in play, standing formally as an individual.
- **`draft_c04_to_02` Sitani Lolohea.** An invented civil engineer. He must first win a Tongatapu people's seat in play. Only then could he be nominated, seconded and recommended by an Assembly majority for appointment as Prime Minister.
- **`draft_c04_to_03` Pisila Tukuafu.** An invented public-finance specialist. A Prime Minister in play could nominate her as a non-elected Minister, within the cap of four, for appointment by the King. Following a general election she remains a caretaker until her appointment is revoked or continued on the incoming Prime Minister's recommendation (clause 51(3)); an election date alone does not dismiss her.
- **`draft_c04_to_04` Kalolo Matalehu.** An invented community-radio producer. He could lead the Democratic Party of the Friendly Islands (PTOA) only under its own unpublished rules and only if its leadership falls vacant. Leading it gives him no seat.

## Unresolved institutional mappings

Full text and evidence are in `proposals.json` → `unresolved_institutional_mappings`.

| ID | Mapping |
| --- | --- |
| UM-01 | Draft IDs are not catalog identities. The runtime derives four `fictional_v1_` slots per row, and saved binding hashes cover name and birth, so the drafts need an additive identity list or new slots rather than renamed templates. |
| UM-02 | No French fictional successor can reach national office today. `future_office_grants` is empty and must name a catalog identity (`presidential_contender`, `authored_fictional_executive_role`). |
| UM-03 | The France engine is single-office: there is no separate presidency, Prime Minister, deputy or group seat. |
| UM-04 | The C01 France packet has not reconciled organizations to the five game rows; this packet relies on roster identity notes. |
| UM-05 | PS (2021), PCF (39th Congress) and RN (2022) statute versions in force at the cutoff were not verified. |
| UM-06 | Tonga has no party rows and is modelled as a non-electoral kingdom (`next: (0, 0)`), so no Tonga draft can ever be selected. The post-2010 elected premiership is unmodelled. |
| UM-07 | Tonga's party pillar in the simulation is "the thirty-three hereditary nobles"; it must never receive fictional people. |
| UM-08 | There is no Tonga executive-role record. If rows were added, successors would default to legacy executive eligibility without review. |
| UM-09 | PTOA's rules are not public, and "Leader" versus "President" is unresolved in C01. |
| UM-10 | There is no collective-seat model for people's seats or the Cabinet; the only seat policy covers two German co-chairs. |

## Observations on the existing contract (no files changed)

- All 60 France templates are `party_only`. Under the current contract no fictional French successor can reach national office.
- The LDP pilot biographies describe undated, already-held Diet membership. This packet is stricter, allowing no pre-cutoff office or membership. C04 should choose one rule for all countries.
- Tonga uses the shared `pacific` name pool, which has had no Tonga review. "Tui" resembles the chiefly title element Tu'i, and "Latu" is the surname of a minister recorded in C01.
- The French pool surname "Faure" is shared with the PS first secretary in the roster (used by existing `fr_fn`/`fr_udf` templates). This is a coincidence flagged for editorial review.

## Sources

**Read — Tonga.**
- Constitution, 2020 Revised Edition: byte-identical to the C01 copy.
- Electoral Act, 2016 Revised Edition.
- Assembly "How Parliament works?" and its 2010 note on the Prime Minister procedure. Both are dynamic pages with hit counters.
- Registry (MCCTIL) incorporated-societies memorandum, 21 July 2026.
- Prime Minister's Office appointment release, 18 December 2025.

**Read — France.**
- Assemblée nationale compilation of July 2025, which contains the Règlement and the Constitution.
- Assemblée fiche n° 3 (September 2023).
- Interior Ministry candidate mementos: legislative 2024 and presidential 2022, both prefecture-hosted copies.
- Senate explanatory statement on presidential sponsorship.
- Élysée English Constitution.
- PS statutes (September 2021) and internal rules (October 2022).
- PCF statutes (39th Congress).
- RN statutes (as modified 5 November 2022).

**Blocked or unavailable and not bypassed.**
- Bot challenges: Légifrance (Code électoral LO127), the Interior Ministry's own pages, info.gouv.fr and vie-publique.fr.
- The Conseil constitutionnel's Constitution and 1962-law pages returned a firewall 403.
- A prefecture page returned an empty reply.
- Two guessed paths returned 404.

As a result, the organic-law texts were read through the ministry mementos, not the consolidated law.

## Validation

```text
python -X utf8 tools/avatars/check_successor_proposals.py
python -X utf8 -m unittest discover -s tools/avatars -p "test_successor_proposals.py"
```

The validator exits `0` on this packet and non-zero on any violation (`2` if inputs are missing). `--list-inputs` prints the SHA-256 of every repository input it read, and `--json` gives a machine report.

The 65 tests mutate the packet to show each check going red:
- historical-window contamination, including dated events, narrative years and pre-cutoff office or party claims;
- missing fictional labels;
- role and age violations, including hereditary roles, excluded rows, sourced minimum ages and plausibility bands;
- duplicate IDs and names;
- collisions with existing fictional IDs, historical IDs, real names, research holders and noble or royal names;
- date-triggered or automatic incumbent replacement.

## Limitations

- **Research scope.** This is preparation research. It does not complete C01 or C02, accept any history or change the research cutoff (still 7 September 2026, pending review).
- **Blocked law texts.** Organic-law details come from official mementos, because the consolidated law was blocked. The 2022 presidential memento predates the cutoff, and later amendments were not verified.
- **Consolidated editions.** The Tonga Constitution and Electoral Act are consolidated editions, so later amendments were not checked. Dynamic pages have non-reproducible hashes.
- **Name screening.** It covers the repository corpus plus lead searches. It cannot rule out private individuals.
- **Not yet decided.** No runtime identity, grant, seat model, candidate list, portrait, prompt or production-manifest entry exists for any draft. Integration depends on the unresolved mappings above and on Codex review.

## Independent bounded integration review

Codex reviewed this submission on 28 September 2026. The [integration packet](../../integrations/CLAUDE-C04-PREP-01/README.md) retains the original submission identities, baseline results, negative regressions, corrected validation, primary-source response recaptures and source-access limits. The validator now fixes the eight-dossier pilot count and reviewed role/gate constraints outside the editable packet. This remains preparation only; no person, artwork, role or succession command is installed. C04, C06 and S23 remain incomplete.
