# CLAUDE-E05-RESEARCH-01 — important-company research pilot

Owner: Claude; reviewer/integrator: Codex. State: **ready_for_review** (submitted 28 September 2026; not reviewed, merged or accepted).
Parent: E05, **research preparation only**; E05 itself is not advanced. Branch: `claude/e05-research-01`. Base: `e41aa18d`;
claim commit `1966fb06`; `codex/campaign-certification` merged at `75626a13` before the delivery commit. Touched paths: only
the owned paths below. See [Result](#result).
Follow the [expanded working contract](CLAUDE-EXPANDED-NEXT.md).

## Build

Research eight important companies, four each in France and Japan, selected from
existing game identities where possible to avoid duplicate businesses. Cover a useful
mix of ground, aerospace/naval and civilian industrial activity without pretending
every company manufactures every platform. Pin source-backed names, country links,
operating periods, mergers/renames and example historical products within 1990 through
the frozen 2026-09-07 research cutoff. Keep later products and businesses explicitly
fictional or unknown; do not predict real company history through 2035.

Use official company histories, annual reports and institutional records. Map each
dossier to proposed existing company/platform IDs with uncertainty flags. Describe
how a later integration could use the existing design → paid development → company
stock → purchase → delivery flow; do not introduce government production capacity,
free stock or a parallel financial ledger. Source facts do not grant logo/image rights.

## Owned paths

New `docs/research/company-pilot/`, `tools/planning/check_company_pilot.py`,
`tools/planning/test_company_pilot.py`; this handoff. Read existing company data/code;
do not change live company identities, balances, production rules, assets or saves.

## Acceptance

Deliver eight sourced machine-readable dossiers, a readable catalog and a validator
with tests for duplicate IDs, unsupported periods, merger identity confusion,
real/fictional mixing and missing provenance. Give each factual claim a source locator
and record inaccessible evidence. Runtime integration and E05 completion stay after
S30/CP1; this assignment authorizes research now, not early expansion mechanics.

## Result

Submitted `ready_for_review` on 28 September 2026. This is research preparation for E05 only. It
adds no company mechanic, and runtime integration stays after S30/CP1.

**Touched paths.** Nothing else was changed:

- `docs/research/company-pilot/README.md` (new): the readable catalog.
- `docs/research/company-pilot/sources.json` (new): 99 official sources, 10 inaccessible attempts
  and 21 leads.
- `docs/research/company-pilot/dossiers/` (new): eight machine-readable dossiers.
- `tools/planning/check_company_pilot.py` (new): the validator. It exits 1 and lists every
  violation.
- `tools/planning/test_company_pilot.py` (new): 57 isolated-fixture tests.
- This record.

No game data, company identity, balance, production rule, asset, save, CI file,
`docs/planning/campaign-pathway.json` or task queue changed. The validator only reads game source.

**Companies and proposed mappings.** All targets are existing identifiers, and each carries an
uncertainty note in its dossier.

| Dossier | Selected because | Proposed targets (confidence) |
|---|---|---|
| `fr_knds_france` (KNDS France; GIAT Industries kept as a separate legal person) | Real counterpart of the French armour supplier programme | `tank_standard` (medium); `ground_artillery`, `ground_ifv` (low); `howitzer_155_guided` ammunition (low); supplier programme `France:ground_apc` (low); `France:defense` contractor (medium) |
| `fr_dassault_aviation` (Dassault Aviation) | Named in France's sourced composite-structures technology | `air_fighter` (medium); `air_tactical_strike` (low); `air_gen4` kit (medium); tech `France:matl_composite_structures` (high, reference only); `France:defense` (medium) |
| `fr_naval_group` (Naval Group; the 1990-2003 DCN directorate kept as a separate entity) | The only French naval identity | `nav_escort` kit (medium); `msl_deterrent` kit (low); `France:defense` (medium) |
| `fr_renault` (Renault Group) | French civilian manufacturer; Nissan alliance link | `France:manufacturing` contractor (medium) |
| `jp_mitsubishi_heavy_industries` (MHI) | Holder of Japan's only supplier programme (aircraft); tanks, F-2, H-IIA/H3, submarines, frigates | `tank_standard`, `air_tactical_strike` (medium); supplier programme `Japan:air_light_attack` (low); `arm_gen3`, `nav_escort` kits (medium); `Japan:defense` (medium) and `Japan:energy` (low) contractors |
| `jp_kawasaki_heavy_industries` (KHI) | Named in Japan's sourced industrial-robotics technology | tech `Japan:matl_industrial_robotics` (high, reference only); `Japan:manufacturing` (medium) and `Japan:defense` (low) contractors |
| `jp_komatsu` (Komatsu) | Japan's wheeled-armour maker (Type 96, LAV, Type 87); no duplicate of MHI tanks | `ground_apc`, `ground_recon` (medium); `howitzer_155_he` ammunition (low); `Japan:manufacturing` (medium) and `Japan:construction` (low) contractors |
| `jp_toyota_motor` (Toyota Motor Corporation) | Named in Japan's sourced lean-production technology; French plant | tech `Japan:matl_lean_production` (high, reference only); `Japan:manufacturing` contractor (medium) |

The catalog and each dossier describe how a later integration could use the existing orders, as
checked against `companies.rs` and `supplier_catalogue.rs`:

1. `Establish`, then `Capitalize`, both paid from Defense procurement.
2. `Develop`: Defense R&D pays development as work is performed.
3. `Inventory`: finite company stock of 0-12 units.
4. `Purchase` of whole units, with delivery seven days after settlement. Exports would use
   `ImportPurchase`; ammunition would use the `Ammo*` orders.

Every dossier sets government production capacity, free stock and a parallel ledger to false.
Opening balances and stock are not sourced and not proposed.

Rulings needed before any integration:

- one real identity per supplier programme;
- platform shape mismatches, such as wheeled CAESAR and VBCI against tracked platforms, and the
  F-2 against the light-attack slot;
- products that have no game consumer: patrol and airlift aircraft, helicopters, launchers,
  airliners, civilian vehicles and machinery;
- the lack of a suitable submarine kit.

**Content.**

- 195 claims, all paraphrased, each with a locator and an anchor of at most 12 words.
- 55 products: 49 historical, 3 announced with outcome unknown after the cutoff, 3 cancelled
  and none fictional.
- 32 proposed mappings.
- The research window is 1990-01-01 to 2026-09-07. No post-cutoff history is predicted.

**Sources.** All 99 sources are official evidence:

- 56 company pages and releases;
- 16 register records (BODACC notices and Japanese National Tax Agency corporate-number records);
- 7 annual reports;
- 5 securities filings (the MHI, KHI and Toyota 有価証券報告書 and Toyota's Form 20-F for the years
  ended 31 March 2017 and 2026);
- 7 Ministry of Defense, ATLA and JGSDF pages;
- 4 French parliamentary documents;
- 2 BnF authority records;
- the Thales universal registration document;
- the Australian National Audit Office sheet for the cancelled Attack class.

Every response was downloaded with a normal browser User-Agent and identity encoding, with its
bytes, SHA-256, content type, final URL and fetch time recorded. Each was re-downloaded 32 to 591
minutes later:

- 65 were byte-stable.
- 34 changed. These are pages generated per request, such as registry pages that print the
  retrieval time and MHI pages. Every claim anchor from them was found again in the recheck.

Every anchor was also machine-checked against the extracted text of the first response. That
covers 195 anchors in French, English and Japanese. Japanese PDFs were read with
`pdftotext -enc UTF-8`.

Some undated JGSDF equipment pages are used. Their only in-window date is the recorded HTTP
`Last-Modified` header, and the claim says so. Source documents are not stored in the repository,
and no logo or image was downloaded. News and encyclopaedia items are leads only.

**Commands and results.** Run on 28 September 2026 in the sparse worktree after the merge of
`75626a13`, with `PYTHONDONTWRITEBYTECODE=1`:

- `python -X utf8 tools/planning/check_company_pilot.py`: PASS, exit 0.
  - Output: 8 dossiers (France 4, Japan 4); 99 official sources (65 byte-stable, 34 changed and
    re-verified); 10 inaccessible recorded; 195 claims; 55 products; 32 proposed mappings.
  - It prints the SHA-256 of each game input it read, LF-normalised. For example,
    `companies.rs` is `b117e441…d04c` and `supplier_catalogue.rs` is `879b9366…c847`.
- `python -X utf8 -m unittest discover -s tools/planning -p "test_company_pilot.py"`: 57 pass.
  - They cover duplicate IDs, unsupported periods (window, cutoff, source coverage, endpoint
    precision, window-start and ongoing evidence), merger identity confusion, real/fictional
    mixing, missing provenance, mappings and forbidden shortcuts, the catalog and the CLI exit
    codes.
  - Ten deliberate validator mutations each turn at least one fixture test red: renames across
    entities, merger recorded as rename, source coverage, fictional label, window, locator, one
    business in two dossiers, fresh evidence, recheck interval and announced delivery.
- `python -X utf8 -m unittest discover -s tools/planning -p "test_*.py"`: 65 pass, so the
  existing workboard tests are unaffected.
- `python -X utf8 tools/planning/workboard.py --check`: PASS.
- `git diff --cached --check` on the owned paths: clean.

**Inaccessible evidence.** All ten are recorded in `sources.json` with time and result; none was
bypassed. The Dassault page was requested twice for its recheck, each time with the same plain
request:

- **HTTP 403:**
  - the 2015 Commission des participations et des transferts opinion on Nexter;
  - Légifrance law 89-924 and decree 90-582;
  - the KHI corporate history and timeline pages.
- **Cloudflare challenge:** the JMSDF naming page.
- **Incapsula bot page on both byte-stability rechecks:** the Dassault Aviation 1986-2000 heritage
  page. It was removed as evidence, so the 1990 Dassault rename is recorded at year precision.
- **HTTP 503:** the BnF catalogue record; the data.bnf.fr record was used instead.
- **Timeout:** the Cour des comptes site.
- **HTTP 404:** one guessed Dassault address.

**Limitations.**

- This is a pilot of eight companies, not an industrial census.
- Some dates are year precision only.
- Some in-service observations rest on page `Last-Modified` dates rather than service-entry years.
- The Mirai is recorded from its launch announcement only.
- Not researched: opening balances, capacity and costs; ownership splits known only from news
  (for example MHI/Hitachi in MHPS); GCAP and MGCS outcomes; Type 16, Soryu and T-4 dates.
- The validator proves that anchors exist and dates are covered. It cannot prove that a paraphrase
  is faithful; the paraphrases were checked by reading the French and Japanese sources.
- Codex review, integration and any E05 mechanics remain open.
