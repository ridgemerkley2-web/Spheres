# Important-company research pilot (E05 preparation)

Task `CLAUDE-E05-RESEARCH-01`. This is **research preparation only** for E05. Nothing here
changes game data, company identities, balances, production rules, assets or saves.
Runtime integration and E05 completion stay after S30/CP1.

- **Research window:** 1990-01-01 through the frozen research cutoff **2026-09-07**.
- **After the cutoff:** no real company history is predicted. Any later product, successor or
  business in a campaign must be authored as explicitly fictional game content, labelled
  "(fictional)", or left unknown. No dossier contains a fictional product.
- **Evidence:** official company histories and releases, annual reports and securities filings,
  official registers, parliaments, defence ministries, a national library and an audit office.
  News and encyclopaedias appear only as leads.
- **Rights:** the facts carry no logo or image rights. No logo, product image or photograph was
  downloaded or stored.

Check the pilot with:

```
python -X utf8 tools/planning/check_company_pilot.py
python -X utf8 -m unittest discover -s tools/planning -p "test_company_pilot.py"
```

## Files

| Path | Contents |
|---|---|
| `sources.json` | 99 official sources. Each records the URL, publisher, kind, access date, `covers_through`, response status, bytes and SHA-256, and a byte-stability recheck. It also lists 10 inaccessible attempts and 21 leads. |
| `dossiers/<id>.json` | One machine-readable dossier per company (schema `spheres-company-pilot-dossier-v1`). |
| `README.md` | This catalog. |

Each dossier records:

- **Entities.** The legal persons involved, with registry numbers and name intervals.
- **Lineage.** Renames stay inside one entity. Mergers, acquisitions, spin-offs, joint ventures
  and stake changes always name their counterparties.
- **Ownership stakes and country links.**
- **Products.** Each product is marked `historical`, `announced` or `cancelled` and has dated
  milestones, each naming the maker.
- **Game mapping.** Proposed targets, each with a confidence and an uncertainty note.
- **Gaps.**
- **Claims.** Every claim names its source, a locator (page, section, record or field), the
  dates it attests and a short locating anchor. Claims are paraphrased.

## The eight companies

| ID | Company | Nation | Activities | Example products in the window | Proposed game targets (confidence) |
|---|---|---|---|---|---|
| `fr_knds_france` | KNDS France | France | ground | Leclerc (service 1992), CAESAR, VBCI, Griffon (first delivery 2019); MGCS announced | `tank_standard` (medium); `ground_artillery`, `ground_ifv` (low); `howitzer_155_guided` ammunition (low); supplier programme `France:ground_apc` (low); `France:defense` contractor (medium) |
| `fr_dassault_aviation` | Dassault Aviation | France | aerospace, civilian industrial | Rafale (operational 2004/2006, 300th in 2025), Falcon 6X (service 2023); Falcon 10X announced | `air_fighter` (medium); `air_tactical_strike` (low); `air_gen4` kit (medium); tech `France:matl_composite_structures` (high, reference only); `France:defense` contractor (medium) |
| `fr_naval_group` | Naval Group | France | naval | Suffren SSN (2020), FDI frigate (2025), Scorpene for Chile; Australian Attack class cancelled 2021 | `nav_escort` kit (medium); `msl_deterrent` kit (low); `France:defense` contractor (medium) |
| `fr_renault` | Renault Group | France | civilian industrial | Clio (1990), Twingo (1992), ZOE (2012), Renault 5 E-Tech (2024) | `France:manufacturing` contractor (medium) |
| `jp_mitsubishi_heavy_industries` | Mitsubishi Heavy Industries | Japan | ground, aerospace, naval, civilian industrial | Type 90 (1991) and Type 10 (2011) tanks, F-2 (1995-2011), H-IIA (to 2025) and H3, Taigei and Mogami (2022); SpaceJet cancelled 2023; Australian frigates announced | `tank_standard`, `air_tactical_strike` (medium); supplier programme `Japan:air_light_attack` (low); `arm_gen3`, `nav_escort` kits (medium); `Japan:defense` (medium) and `Japan:energy` (low) contractors |
| `jp_kawasaki_heavy_industries` | Kawasaki Heavy Industries | Japan | aerospace, naval, civilian industrial | P-1 (2013), C-2 (2016), OH-1 (1996), Hakugei (2023), 787 fuselages, Shinkansen and export trains, Ninja H2 (2015), R series robots (2008) | tech `Japan:matl_industrial_robotics` (high, reference only); `Japan:manufacturing` (medium) and `Japan:defense` (low) contractors |
| `jp_komatsu` | Komatsu | Japan | ground, civilian industrial | Type 96 APC, Light Armored Vehicle, Type 87 reconnaissance vehicle, autonomous haulage and hybrid excavator (2008); Wheeled APC (Improved) cancelled 2018 | `ground_apc`, `ground_recon` (medium); `howitzer_155_he` ammunition (low); `Japan:manufacturing` (medium) and `Japan:construction` (low) contractors |
| `jp_toyota_motor` | Toyota Motor Corporation | Japan | civilian industrial, ground | Carina E at TMUK (1992), Mega Cruiser (1996-2001), Prius (1997), Yaris at TMMF France (2001), Mirai (2014), JGSDF High Mobility Vehicle | tech `Japan:matl_lean_production` (high, reference only); `Japan:manufacturing` contractor (medium) |

Together they cover ground, aerospace, naval and civilian industrial activity. No company is
credited with platforms it did not make. Several links cross the two countries: Renault and
Nissan, Toyota's French plant, and Mitsubishi Motors (not MHI) in the Renault alliance.

### Why these companies

The game has no real company identities. Companies are created at runtime through
`CompanyOrder::Establish`; sector contractors are generated and fictional; the supplier
programmes are modeled "Export Works" firms. Each company was therefore chosen where the game
already points at it, or where it prevents a duplicate business:

- **Existing game references.**
  - The supplier programmes `France:ground_apc` and `Japan:air_light_attack`
    (`spheres-sim/src/supplier_catalogue.rs`) are proposed, at low confidence, for KNDS
    France and MHI.
  - Japan's sourced 1990 technology evidence names Kawasaki Heavy Industries
    (`matl_industrial_robotics`) and the Toyota Production System (`matl_lean_production`).
  - France's evidence names Dassault (`matl_composite_structures`).
- **Duplicate-avoiding choices.**
  - Naval Group is the only French naval identity.
  - Komatsu is the only Japanese wheeled-vehicle identity, so MHI's tanks are not duplicated.
  - Renault and Toyota are the civilian manufacturers.

### Company notes

- **KNDS France.** The pilot keeps two legal persons apart:
  - GIAT Industries (RCS 352 751 143) is the state company of 1990 and later a holding.
  - Nexter Systems (RCS 379 706 344) became KNDS France on 8 April 2024.

  Nexter is not treated as a rename of GIAT Industries. The name of 379 706 344 before
  December 2006 is a recorded gap.
- **Dassault Aviation.** Avions Marcel Dassault-Breguet Aviation took the name Dassault Aviation
  in 1990, recorded at year precision. The company heritage page gives the day, but it failed
  both byte-stability rechecks behind bot protection and is recorded as inaccessible. Thales,
  in which Dassault holds a stake, stays a separate company.
- **Naval Group.** The 1990-2003 state directorate (DCN) is its own entity; it is not the
  company and not government production capacity. The company then carried four names: DCN
  Développement, DCN, DCNS and Naval Group.
- **Renault Group.** The Régie became Renault SA in 1990; privatisation took effect in 1996.
  - The Nissan alliance (1999) is a shareholding, not a merger.
  - Mitsubishi joined the alliance in 2016; that is Mitsubishi Motors, not MHI.
- **Mitsubishi Heavy Industries.** MHI kept one name through the window. Mitsubishi Hitachi
  Power Systems and Mitsubishi Power are one legal person under two names; its thermal power
  business was absorbed into MHI on 1 October 2021. Separate companies:
  - Mitsubishi Aircraft, whose SpaceJet was discontinued on 7 February 2023.
  - Mitsubishi Shipbuilding (2018), which is not the pre-1964 company of the same Japanese name.
  - Mitsubishi Motors, independent since 1970.

  The Australian General Purpose Frigate contract (18 April 2026) is `announced` and its
  outcome is unknown. Its December 2029 delivery target is not recorded as history.
- **Kawasaki Heavy Industries.** Kawasaki Shipbuilding Corporation (2002-2010) is its own
  entity; it split off in October 2002 and merged back in October 2010. Kawasaki Railcar
  Manufacturing and Kawasaki Motors were split off in October 2021. KHI sold 20% of Kawasaki
  Motors on 1 April 2025, and ITOCHU took that stake. Separate companies, not KHI businesses:
  - K Line and Kawasaki Steel.
  - The 1896 founding company, 株式会社川崎造船所.
  - Kawasaki Motors Corp., U.S.A.
- **Komatsu.** The registered name is 株式会社小松製作所 (brand コマツ). A company with the
  identical trade name in Suwa, Nagano is a different legal person. After the 2018
  cancellation of the Wheeled Armored Vehicle (Improved), the Ministry of Defense chose
  Patria's AMV in 2022. Komatsu's FY2025 report still lists ammunition and armoured personnel
  carriers.
- **Toyota Motor Corporation.** The present company dates from the 1982 merger. These are
  separate legal persons, not TMC divisions:
  - Toyota Industries (forklifts since 2001).
  - Daihatsu (wholly owned since 2016).
  - Hino (consolidated 2001 to 31 March 2026).

  The Toyota Fudosan tender offer for Toyota Industries (24 March 2026) is recorded. The later
  squeeze-out and sale are not verified at the cutoff.

## How a later integration could use the existing company flow

This section describes current code only; it proposes no mechanic. It follows
`spheres-sim/src/companies.rs` (`CompanyOrder`) and `spheres-sim/src/supplier_catalogue.rs`.
A reviewed, post-CP1 integration could attach a sourced identity to a company that goes
through the existing orders:

1. **`Establish`.** Defense procurement capitalises the company, and a political-capital price
   applies. The company leases one free, completed Arms Plant slot. No historical factory,
   cash or stock is granted.
2. **`Capitalize`.** Defense procurement adds working capital. Capital becomes spendable only
   after fiscal settlement.
3. **`Develop`.** The design stays with the commissioning country and is frozen as a revision.
   Defense R&D pays engineering, prototype and trials work as it is performed. The company
   funds tooling, national-warehouse inputs and fabrication from its own settled cash.
   Prototypes are never saleable.
4. **`Inventory`.** A finite stock target of 0-12 units. A target is not a government order.
5. **`Purchase`.** The government buys whole finished units from company stock. Delivery comes
   seven accessible days after fiscal settlement. Exports would use `ImportPurchase` from
   company stock, never a scripted contract.

- **Ammunition** would use `AmmoSupply`, `AmmoInventory` and `AmmoPurchase` for a licensed
  family.
- **Sector-contractor slots** would only replace one generated, fictional contractor. They
  improve work that another ledger funds.
- **Arsenal kits and technology nodes** have no company consumer. Kits are flavour references
  and technology nodes are identity references only.

Every dossier sets the three forbidden shortcuts to false: government production capacity,
free or opening stock, and a parallel financial ledger. Opening balances and stock stay
unsourced and unproposed. The validator checks every proposed target against current game
source and every integration step against the current `CompanyOrder` variants.

Uncertainties that need a ruling before integration:

- **One real identity per supplier programme.**
  - `France:ground_apc` would take KNDS France.
  - `Japan:air_light_attack` would take MHI. The F-2 is a larger support fighter; no Japanese
    light-attack type was researched.
- **Platform shapes.** Several platform envelopes differ from the real products: wheeled CAESAR
  and VBCI against tracked game platforms, and the F-2 against the strike airframe.
- **Missing product consumers.** Many real products have none:
  - maritime patrol and airlift aircraft, and helicopters;
  - launch vehicles and airliners;
  - civilian cars, trains, robots and machinery.
- **Submarines.** No submarine kit fits. `aip_ssk` assumes air-independent propulsion, which
  is not sourced for Taigei, Hakugei or Scorpene; `la_ssn` is the US Los Angeles class.

## Provenance

- **Recording.** Every source was downloaded with a normal browser User-Agent and identity
  encoding. Its bytes and SHA-256 are recorded.
- **Rechecks.** Each source was downloaded again more than 30 minutes later, and 65 responses
  were byte-stable. The other 34 changed, mostly because the page is generated per request:
  - registry pages print the retrieval time;
  - MHI pages rebuild per request.

  Every claim anchor from those 34 was found again in the recheck response.
- **Anchors.** Each anchor was machine-checked against the extracted text of the saved first
  response. HTML text came from the visible text and the page's embedded data; PDFs from
  `pdftotext -enc UTF-8`; ESEF packages from their XHTML.
- **Page locators** are physical PDF pages, counted from 1.
- **Documents are not stored in the repository.**
- **Evidence reached through response metadata.** Some JGSDF equipment pages are undated. For
  those, the only in-window date is the HTTP `Last-Modified` header of the recorded response,
  and the claim says so. Registry pages retrieved on 2026-09-28 are used only for the dated
  record contents inside the window.

### Inaccessible evidence

All ten attempts are recorded in `sources.json` under `inaccessible`. None was retried to get
around a block, and none supports a claim.

- **Blocked with HTTP 403:**
  - the Commission des participations et des transferts opinion on Nexter (2015);
  - Légifrance texts of law 89-924 and decree 90-582;
  - KHI's corporate history and timeline pages.
- **Bot challenges:**
  - a JMSDF naming page answered with a Cloudflare challenge;
  - the Dassault Aviation heritage page (1986-2000) answered both byte-stability rechecks with
    an Incapsula bot page, so it was removed as evidence.
- **Other failures:**
  - the BnF catalogue record for GIAT Industries returned HTTP 503 (the data.bnf.fr record was
    used instead);
  - the Cour des comptes site timed out;
  - one guessed Dassault address returned HTTP 404.

## Limitations

- **Scope.** This is a pilot of eight companies. It proves the method and the validator; it is
  not a complete industrial map of either country.
- **Dates.**
  - Some dates are year precision only: the Dassault and Renault 1990 changes, Rafale service
    entry, and several KHI and MHI timeline entries.
  - Some Japanese in-service observations rest on page `Last-Modified` dates rather than
    service-entry dates.
  - The Mirai is recorded from its launch announcement only.
- **Not researched:**
  - opening balances, workforce, plant capacity and unit costs;
  - ownership splits reported only by news, such as MHI/Hitachi in MHPS;
  - later GCAP and MGCS outcomes;
  - Type 16, the Soryu class, the T-4 trainer and other products without official dated evidence.
- **Validator coverage.** The offline validator checks that a locating anchor is recorded,
  provenance metadata is complete, dates match the declared source coverage, and game targets
  exist. It does not download source bodies or prove that an anchor occurs in a source, that a
  response hash reproduces, or that a paraphrase is faithful. Source retrieval and content
  review are separate checks. The original researcher reports reading the French and Japanese
  sources directly; independent access limits are recorded in the integration review.
- **Name periods.** Legal rename endpoints are exclusive: the new name applies on an exact
  rename day. An endpoint explicitly marked `last_observed` includes the observation day and
  does not assert that the company or name ceased to exist then. Year-only dates retain their
  uncertainty; they do not resolve which day in that year a name changed.
