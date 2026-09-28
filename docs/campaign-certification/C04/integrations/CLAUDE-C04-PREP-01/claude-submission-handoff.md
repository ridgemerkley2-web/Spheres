# CLAUDE-C04-PREP-01 — fictional successor pilot

Owner: Claude. State: **ready_for_review** (27 September 2026). This is a preparation submission only: it is not merged, accepted or C04-complete. Parent: C04, **preparation only**.
Branch: `claude/c04-prep-01`. Original base: `e41aa18d`, merged forward to `846df479` in `08a18aee`. Claim commit: `f1547abd`. Deliverables: `63389675`; this record's update is the following commit.
Touched paths: only the owned paths below. Next step: Codex review of the packet, validator and tests. Integration decisions stay with Codex (see Result).
Follow the [expanded working contract](CLAUDE-EXPANDED-NEXT.md).

## Build

Create eight proposal-only fictional successor dossiers, four each for France and
Tonga, grounded in sourced institutional and political career patterns. Distinguish
party leadership, executive eligibility, hereditary offices and collective institutions;
do not invent an elected-party succession path for a hereditary office. Use primary
institutional sources with access dates, content locators and uncertainty notes.

Each proposal needs a stable draft ID, explicitly invented name/biography, compatible
role, plausible age/career constraints, potential eligibility window after 2026-09-07
through 2035-12-31, and cartoon appearance brief. An eligibility window is never a
predicted appointment or automatic incumbent replacement. Do not assign invented
people to real historical office records or manufacture historical parentage.

## Owned paths

New `docs/campaign-certification/C04/preparation/france-tonga/`,
`tools/avatars/check_successor_proposals.py`,
`tools/avatars/test_successor_proposals.py`; this handoff.
Read current research and succession contracts; no runtime rules, candidates,
portrait installation or production manifest changes.

## Acceptance

Deliver eight clearly labeled draft dossiers, sources, a human-readable review and
an executable validator. Test historical-window contamination, missing fictional
labels, role/age violations, duplicate IDs and date-triggered incumbent replacement
in the proposed contract. Record unresolved institutional mappings. Full C04 still
requires accepted C02 inputs, reviewed artwork and later integration.

## Result — ready_for_review (27 September 2026)

Proposal-only preparation. Eight **invented** characters, four for France and four for Tonga, with sourced institutional constraints, a readable review, and a validator with tests. Nothing is installed, granted, illustrated or scheduled.

### Touched paths

| Path | Status |
| --- | --- |
| `docs/campaign-certification/C04/preparation/france-tonga/proposals.json` | new; SHA-256 `cc2882ccda191ac69169bebca2193b99971903bd109dac51e23aedce73fa5d01` |
| `docs/campaign-certification/C04/preparation/france-tonga/sources.json` | new; SHA-256 `1260efe01c4688ac284d47e15c409158b22fd5057980d66798df1585ef77492a` |
| `docs/campaign-certification/C04/preparation/france-tonga/README.md` | new; the readable review |
| `tools/avatars/check_successor_proposals.py` | new; the validator |
| `tools/avatars/test_successor_proposals.py` | new; 65 tests |
| this handoff | updated |

No runtime rule, candidate list, portrait, prompt, production manifest, research packet, index, CI file, `campaign-pathway.json` or task queue was edited.

### Dossiers

Every draft has these properties:
- **Labels and status:** labelled fictional; status `proposal_only_not_installed`.
- **Background:** undated, private and professional only. There is no pre-cutoff office, party membership or candidacy.
- **Lineage:** parentage `not_specified`; no title or descent.
- **Succession contract:** `actual_succession_events_only`, incumbents retained, no date trigger, no predicted appointment, no automatic acting role.
- **Appearance:** a text-only cartoon brief; no image was generated.

| Draft ID | Invented name | Role | Row | Window |
| --- | --- | --- | --- | --- |
| `draft_c04_fr_01` | Ondine Rivasseau | PS First Secretary (party leadership) | `fr_ps` | 2029-09-08 → 2035-12-31 |
| `draft_c04_fr_02` | Loïc Ferrandou | PCF National Secretary (party leadership) | `fr_pcf` | 2026-09-08 → 2035-12-31 |
| `draft_c04_fr_03` | Gwenola Pérignac | Presidential contender (executive eligibility) | `fr_ps` | 2029-09-08 → 2035-12-31 |
| `draft_c04_fr_04` | Anatole Villaume | National Assembly deputy (collective institution) | `fr_fn` | 2026-09-08 → 2035-12-31 |
| `draft_c04_to_01` | Lesieli Fotu | People's representative (collective institution) | none | 2026-09-08 → 2035-12-31 |
| `draft_c04_to_02` | Sitani Lolohea | Prime Minister via a people's seat (executive eligibility) | none | 2026-09-08 → 2035-12-31 |
| `draft_c04_to_03` | Pisila Tukuafu | Non-elected Cabinet Minister (collective institution) | none | 2026-09-08 → 2035-12-31 |
| `draft_c04_to_04` | Kalolo Vaikona | PTOA leader (party leadership) | none | 2026-09-08 → 2035-12-31 |

**Why the two PS windows open in 2029.** No membership may predate the cutoff. The PS requires three consecutive years of membership for the national bodies the First Secretary presides over (art. 2.6.5) and for national candidacies (art. 5.1.5).

**Sourced minimum ages.**
- PS membership: 15.
- President and deputies: 18.
- Tongan people's seats, and therefore the Prime Minister route through them: 21.
- Where no legal age exists, the roles use authored plausibility bands.

**Tonga hereditary offices are excluded with sourced reasons.** A fictional holder of any of these would manufacture parentage or a peerage:
- the Crown (clauses 32 and 79);
- the Royal Family and regency (clauses 27 and 42-43);
- nobles (clauses 44 and 111);
- nobles' representatives (clauses 60 and 63);
- the Speaker (clause 61).

The Privy Council is excluded as a royal-pleasure body. The nobles' route to the premiership, documented in the Prime Minister's Office release of December 2025, is not used.

**France.** There is no hereditary office (art. 89). The `fr_rpr` and `fr_udf` rows are excluded because their roster identity notes forbid invented post-2002 and post-2007 successor chains.

### Sources

`sources.json` holds 28 records with 67 claims, each with a locator and uncertainty note, plus a 30-request fetch log. Every record read carries the bytes and SHA-256 of the response. Downloads were deleted after reading.

**Read — Tonga (6).**
- Constitution, 2020 Revised Edition: byte-identical to the C01 copy.
- Electoral Act, 2016 Revised Edition.
- Assembly pages "How Parliament works?" and the Prime Minister procedure note. Both have hit counters, so their hashes are not reproducible.
- MCCTIL incorporated-societies memorandum, 21 July 2026.
- Prime Minister's Office release, 18 December 2025.

**Read — France (12).**
- Assemblée nationale compilation of July 2025, containing the Règlement and the Constitution.
- Assemblée fiche n° 3.
- Interior Ministry candidate mementos: legislative 2024 and presidential 2022, both prefecture-hosted copies.
- Senate explanatory statement.
- Élysée English Constitution.
- PS statutes (2021) and internal rules (2022), plus the PS reference page.
- PCF 39th Congress statutes, plus the PCF page.
- RN statutes (2022).

**Blocked or unavailable and not bypassed.**
- Bot challenges: Légifrance LO127, two Interior Ministry pages, info.gouv.fr and vie-publique.fr.
- The Conseil constitutionnel's Constitution and 1962-law pages returned a firewall 403.
- The Haute-Loire prefecture page was unreachable.
- Two guessed paths returned 404.

### Commands and results

These were run on the merged tree (`08a18aee`) with the deliverables:

```text
python -X utf8 tools/avatars/check_successor_proposals.py --list-inputs
  PASS: 8 proposal-only fictional drafts (France 4, Tonga 4); 28 sources; 878 real names
  and 3146 existing IDs checked; windows within 2026-09-08..2035-12-31; no hereditary role,
  date trigger or installation.   (exit 0)
python -X utf8 -m unittest discover -s tools/avatars -p "test_successor_proposals.py"
  Ran 65 tests ... OK
python tools/planning/workboard.py --check
  PASS: 44 canonical markers, each assigned once; handoffs and dependencies exist.
  No status is duplicated. 17 bounded tasks validated separately.
git diff --cached --check   (owned paths)   clean
```

**Input hashes.** The contract inputs read match the source hashes recorded in `C01/census.json`:

| Input | SHA-256 prefix |
| --- | --- |
| `future_candidates_2035.json` | `75afd1f5` |
| `leadership_production_2035.json` | `f98cbae9` |
| `fictional_portraits.json` | `54a16634` |
| `party_leaders.json` | `18774ffa` |
| `future_party_leadership.json` | `96e724b2` |
| `future_party_leadership_seats.json` | `cbf74d65` |
| `party_executive_eligibility.json` | `d4fbdbff` |

`--list-inputs` prints all 45 inputs with full hashes.

**Tests.** Each test mutates the packet and asserts the specific error it should raise:
- historical-window contamination: the window start, dated events, narrative years, pre-cutoff office or party claims, and dated backgrounds;
- missing fictional labels;
- role and age violations: hereditary and unknown roles, nation or category mismatch, minimum and maximum ages, excluded or foreign party rows;
- duplicate IDs and names;
- collisions with existing fictional IDs, historical IDs, real census names, close resemblances, research holders, research-text surnames, template names and pool combinations, Tongan noble or royal names, and honorifics;
- real people or historical term records mentioned in a proposal;
- date-triggered or automatic incumbent replacement: contract fields, dated keys, gates without a vacancy or election, date gates;
- sources, appearance, installation and the packet header.

Three command-line tests cover exit codes 0, 1 and 2.

**Name screening.** The final names have no match in the repository corpus, which includes each nation's research record. A lead web search found no public-figure match. Ten earlier candidates were discarded for identical or near-identical real people.

### Unresolved institutional mappings

Details are in `proposals.json` and the review.

| ID | Mapping |
| --- | --- |
| UM-01 | Drafts are not `fictional_v1_` catalog identities: there are four slots per row, and binding hashes cover name and birth. |
| UM-02 | France needs a `presidential_contender` future-office grant, which requires a catalog identity; `future_office_grants` is empty. |
| UM-03 | The France engine is single-office: there is no presidency, Prime Minister, deputy or group seat. |
| UM-04 | The C01 France packet has no organization-to-row reconciliation. |
| UM-05 | PS, PCF and RN statute versions at the cutoff were not verified. |
| UM-06 | Tonga has no party rows and a non-electoral polity model (`next: (0, 0)`). |
| UM-07 | Tonga's hereditary-nobles pillar must never receive fictional people. |
| UM-08 | There is no Tonga executive-role record: a legacy default would apply if rows were added. |
| UM-09 | PTOA's rules are not public, and "Leader" versus "President" is unresolved. |
| UM-10 | There is no collective-seat model; the only seat policy covers two German co-chairs. |

The review also records these observations; no files were changed:
- The 60 France templates are all `party_only`.
- The LDP pilot allows pre-cutoff office in biographies.
- The shared `pacific` pool used for Tonga includes "Tui" and "Latu".
- The French pool uses "Faure".

### Limitations

- **Blocked law texts.** Organic-law details come from official mementos, because Légifrance and the Conseil constitutionnel were blocked. The 2022 presidential memento and the party statute versions were not re-verified against the cutoff date.
- **Consolidated editions.** The Tonga Constitution and Electoral Act are consolidated editions, so later amendments were not checked. Dynamic-page hashes are not reproducible.
- **Name screening.** It cannot rule out private individuals, so editorial review is required.
- **No runtime artifacts.** No runtime identity, grant, seat model, candidate entry, portrait or prompt exists for any draft.

**Full C04 still requires:**
- accepted C02 inputs for France and Tonga;
- reviewed artwork under the C03 standard;
- decisions on UM-01 to UM-10;
- later integration by Codex.

This packet does not complete C04, accept any history, change the 7 September 2026 cutoff or award a certificate.
