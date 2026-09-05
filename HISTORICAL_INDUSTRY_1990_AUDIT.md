# Historical industry at the 1990 start: coverage audit

This is the **before-change audit**. The subsequently approved, explicitly modeled
new-campaign baseline is documented in [STARTING_INDUSTRY.md](STARTING_INDUSTRY.md).
The historical-data gaps below are not erased by introducing game estimates.

Date: 2026-09-04. Scope: the current local working tree, canonical 1 January 1990 initialization, and existing GDP/production contracts. This is a local audit and proposed integration plan, **not a historical factory dataset or an implemented feature**. No game state, starting data, save, or simulation code was changed for this report.

## Finding

All **137 canonical starting countries** have macroeconomic GDP, but **none has a historical factory inventory in the initial physical-production ledger**. Missing inventory is not evidence of zero factories in the real country.

The opening world has **0 completed full-size sites, 0 fractional starter-module capacity, 0 pending construction projects, 0 directed manufacturing lines, and 0 intermediate/capital-goods inventory**, both per country and worldwide. Existing military equipment and national raw-resource production are separate starting systems; neither is a factory census. Player/AI construction later creates tracked physical assets; that does not reconstruct what existed in 1990.

Evidence:

- `spheres-sim/src/init.rs:19`: `world_1990` loads embedded nation/relationship data and initializes governments.
- `spheres-sim/src/data/mod.rs:842`, especially `:875–882`: production/manufacturing and industry-related ledgers start empty/default. Technology endowments are loaded separately.
- `spheres-sim/src/production.rs:213`, `spheres-sim/src/industry.rs:130`, `spheres-sim/src/manufacturing.rs:53`: default completed-site, module, goods, project, and line collections are empty.
- `spheres-sim/src/data/mod.rs:108` and `:186`: the nation and economy schemas have macroeconomic fields and technology provenance, but no industrial establishment counts, capacity inventory, or historical sector mix.

## Coverage and sourcing limitations

The canonical ordered list is `spheres-sim/src/data/embedded.rs:14`, not every nation ID in the executable and not an unordered directory scan. Its 137 files contain 137 unique IDs and all have a top-level `sources` block. **A source block is not verification of every number in its file.**

The table below copies opening `economy.gdp_bn` exactly as stored; the schema calls this real billions of 1990 US dollars. It is not a fresh external validation of GDP, exchange-rate conversion, price basis, territorial coverage, or rounding. Country notes include transcriptions, constructions, estimates, proxies, and limitations. Examples: `data/nations/czechoslovakia.json` explicitly constructs its GDP; `tonga.json` cites GDP but separately identifies estimated fiscal inputs; `madagascar.json` and four related files disclose reading WDI through an aggregator. Technology-specific citations and later sourced population-growth/reserve fields do not retrospectively source an inherited GDP value.

### Correction to the apparent “19 origin-gap countries”

**16**, not 19, canonical countries contain their own explicit note beginning “No per-nation sourcing note existed”:

USA, USSR, China, Japan, Germany, UK, France, Italy, India, Pakistan, Iraq, Kuwait, SaudiArabia, Iran, SouthKorea, Poland.

They are marked **G** below. See `spheres-sim/data/nations/usa.json:31–32`, `japan.json:31`, and `poland.json:31`; the remaining marked files carry the corresponding note in their top-level `sources`.

The broader phrase search produces 19 file hits because **UAE, Oman, and Bahrain mention Saudi Arabia/Kuwait's inherited provenance problem**, not a self-declared original-data gap. Qatar makes the same comparison with different wording. Counting those references as new affected countries misclassifies the notes. Conversely, the other 121 countries are **not certified GDP observations** simply because this particular self-gap note is absent. A field-by-field source review remains necessary.

### Province coverage is a map count, not a historical administrative census

The current starting ownership map has **2,584 unique districts**, no duplicate ownership among canonical starters, and **131 mapped countries**. Bahrain, Mauritius, Seychelles, Comoros, CapeVerde, and Maldives have **no mapped starting district**. Their national GDP remains explicit unallocated GDP; their absence from this map does not mean zero land, population, or industry.

Counts below follow `spheres-sim/data/districts.json` and the starter-only ownership construction in `spheres-sim/src/districts.rs:481` / `:675`. Federation successor lists are not counted a second time. These are game-map districts: the artifact cites Natural Earth administrative geometry plus generated terrain/adjacency, and this audit does not certify every boundary or province count as historically correct on 1 January 1990.

Province population uses the resource artifact's 1990 population entries over a GHS-derived fallback surface, not an industrial survey (`spheres-sim/src/districts.rs:491–527`; `spheres-web/data/district_population.json`). Population weighting cannot locate factories or measure regional productivity.

## Full canonical coverage table

GDP is the stored opening national value in **$bn**, not verified historical industrial output. Province count is current starting ownership. **Every row has missing historical factory inventory and zero tracked initial full/fractional/pending factory capacity**; that common status is stated here rather than repeated 137 times. **G** is the self-origin-gap note above; **—** means only that this note is absent. Country links identify the exact input file.

| Canonical country ID | Opening GDP ($bn) | Map provinces | Origin note |
|---|---:|---:|:---:|
| [USA](spheres-sim/data/nations/usa.json) | 5980 | 51 | G |
| [USSR](spheres-sim/data/nations/ussr.json) | 1600 | 269 | G |
| [China](spheres-sim/data/nations/china.json) | 390 | 32 | G |
| [Japan](spheres-sim/data/nations/japan.json) | 3140 | 47 | G |
| [Germany](spheres-sim/data/nations/germany.json) | 1710 | 16 | G |
| [UK](spheres-sim/data/nations/uk.json) | 1090 | 16 | G |
| [France](spheres-sim/data/nations/france.json) | 1270 | 18 | G |
| [Italy](spheres-sim/data/nations/italy.json) | 1180 | 20 | G |
| [India](spheres-sim/data/nations/india.json) | 320 | 36 | G |
| [Pakistan](spheres-sim/data/nations/pakistan.json) | 40 | 8 | G |
| [Iraq](spheres-sim/data/nations/iraq.json) | 60 | 18 | G |
| [Kuwait](spheres-sim/data/nations/kuwait.json) | 18 | 6 | G |
| [SaudiArabia](spheres-sim/data/nations/saudiarabia.json) | 117 | 13 | G |
| [Iran](spheres-sim/data/nations/iran.json) | 120 | 31 | G |
| [SouthKorea](spheres-sim/data/nations/southkorea.json) | 280 | 17 | G |
| [Poland](spheres-sim/data/nations/poland.json) | 66 | 16 | G |
| [Brazil](spheres-sim/data/nations/brazil.json) | 385 | 27 | — |
| [Indonesia](spheres-sim/data/nations/indonesia.json) | 106 | 33 | — |
| [Egypt](spheres-sim/data/nations/egypt.json) | 43 | 27 | — |
| [Israel](spheres-sim/data/nations/israel.json) | 62 | 6 | — |
| [Turkey](spheres-sim/data/nations/turkey.json) | 151 | 81 | — |
| [Nigeria](spheres-sim/data/nations/nigeria.json) | 54 | 37 | — |
| [Vietnam](spheres-sim/data/nations/vietnam.json) | 6.5 | 9 | — |
| [Yugoslavia](spheres-sim/data/nations/yugoslavia.json) | 88 | 106 | — |
| [Spain](spheres-sim/data/nations/spain.json) | 535 | 19 | — |
| [Netherlands](spheres-sim/data/nations/netherlands.json) | 318.8 | 15 | — |
| [Belgium](spheres-sim/data/nations/belgium.json) | 205.3 | 11 | — |
| [Sweden](spheres-sim/data/nations/sweden.json) | 261.5 | 21 | — |
| [Switzerland](spheres-sim/data/nations/switzerland.json) | 269.8 | 26 | — |
| [Austria](spheres-sim/data/nations/austria.json) | 165.8 | 9 | — |
| [Portugal](spheres-sim/data/nations/portugal.json) | 78.7 | 20 | — |
| [Greece](spheres-sim/data/nations/greece.json) | 96.5 | 14 | — |
| [Denmark](spheres-sim/data/nations/denmark.json) | 138.2 | 5 | — |
| [Norway](spheres-sim/data/nations/norway.json) | 119.3 | 21 | — |
| [Finland](spheres-sim/data/nations/finland.json) | 141.4 | 18 | — |
| [Ireland](spheres-sim/data/nations/ireland.json) | 49.3 | 8 | — |
| [Czechoslovakia](spheres-sim/data/nations/czechoslovakia.json) | 50 | 22 | — |
| [Hungary](spheres-sim/data/nations/hungary.json) | 34.5 | 7 | — |
| [Romania](spheres-sim/data/nations/romania.json) | 38.3 | 42 | — |
| [Bulgaria](spheres-sim/data/nations/bulgaria.json) | 20.7 | 28 | — |
| [Albania](spheres-sim/data/nations/albania.json) | 2.1 | 12 | — |
| [Argentina](spheres-sim/data/nations/argentina.json) | 141.4 | 24 | — |
| [Mexico](spheres-sim/data/nations/mexico.json) | 261.3 | 33 | — |
| [Chile](spheres-sim/data/nations/chile.json) | 33.4 | 16 | — |
| [Colombia](spheres-sim/data/nations/colombia.json) | 47.8 | 34 | — |
| [Venezuela](spheres-sim/data/nations/venezuela.json) | 48.6 | 26 | — |
| [Peru](spheres-sim/data/nations/peru.json) | 26.4 | 26 | — |
| [Cuba](spheres-sim/data/nations/cuba.json) | 28.6 | 16 | — |
| [Bolivia](spheres-sim/data/nations/bolivia.json) | 4.87 | 9 | — |
| [Ecuador](spheres-sim/data/nations/ecuador.json) | 15.2 | 24 | — |
| [Uruguay](spheres-sim/data/nations/uruguay.json) | 9.3 | 19 | — |
| [Syria](spheres-sim/data/nations/syria.json) | 12.31 | 15 | — |
| [Jordan](spheres-sim/data/nations/jordan.json) | 4.16 | 12 | — |
| [Lebanon](spheres-sim/data/nations/lebanon.json) | 2.84 | 6 | — |
| [UAE](spheres-sim/data/nations/uae.json) | 50.7 | 9 | — |
| [Qatar](spheres-sim/data/nations/qatar.json) | 7.36 | 7 | — |
| [Oman](spheres-sim/data/nations/oman.json) | 13.31 | 11 | — |
| [Yemen](spheres-sim/data/nations/yemen.json) | 12.64 | 21 | — |
| [Bahrain](spheres-sim/data/nations/bahrain.json) | 4.81 | 0 | — |
| [Algeria](spheres-sim/data/nations/algeria.json) | 62 | 48 | — |
| [Morocco](spheres-sim/data/nations/morocco.json) | 30.2 | 16 | — |
| [Tunisia](spheres-sim/data/nations/tunisia.json) | 12.3 | 23 | — |
| [Libya](spheres-sim/data/nations/libya.json) | 28.9 | 22 | — |
| [Sudan](spheres-sim/data/nations/sudan.json) | 33.6 | 17 | — |
| [SouthAfrica](spheres-sim/data/nations/southafrica.json) | 126 | 9 | — |
| [Ethiopia](spheres-sim/data/nations/ethiopia.json) | 12.5 | 17 | — |
| [Kenya](spheres-sim/data/nations/kenya.json) | 8.6 | 8 | — |
| [Ghana](spheres-sim/data/nations/ghana.json) | 5.9 | 10 | — |
| [Zaire](spheres-sim/data/nations/zaire.json) | 9.3 | 11 | — |
| [Angola](spheres-sim/data/nations/angola.json) | 11.2 | 18 | — |
| [Zimbabwe](spheres-sim/data/nations/zimbabwe.json) | 8.8 | 10 | — |
| [Tanzania](spheres-sim/data/nations/tanzania.json) | 6.2 | 30 | — |
| [Uganda](spheres-sim/data/nations/uganda.json) | 4.3 | 4 | — |
| [Senegal](spheres-sim/data/nations/senegal.json) | 7.4 | 14 | — |
| [Cameroon](spheres-sim/data/nations/cameroon.json) | 12.3 | 10 | — |
| [Bangladesh](spheres-sim/data/nations/bangladesh.json) | 31.6 | 7 | — |
| [SriLanka](spheres-sim/data/nations/srilanka.json) | 8 | 9 | — |
| [Nepal](spheres-sim/data/nations/nepal.json) | 3.6 | 14 | — |
| [Afghanistan](spheres-sim/data/nations/afghanistan.json) | 3 | 34 | — |
| [Myanmar](spheres-sim/data/nations/myanmar.json) | 11 | 14 | — |
| [NorthKorea](spheres-sim/data/nations/northkorea.json) | 23.1 | 11 | — |
| [Taiwan](spheres-sim/data/nations/taiwan.json) | 166.6 | 21 | — |
| [Mongolia](spheres-sim/data/nations/mongolia.json) | 2.56 | 22 | — |
| [Thailand](spheres-sim/data/nations/thailand.json) | 85.3 | 6 | — |
| [Malaysia](spheres-sim/data/nations/malaysia.json) | 44 | 16 | — |
| [Singapore](spheres-sim/data/nations/singapore.json) | 36.1 | 5 | — |
| [Philippines](spheres-sim/data/nations/philippines.json) | 50.5 | 17 | — |
| [Cambodia](spheres-sim/data/nations/cambodia.json) | 1.4 | 24 | — |
| [Laos](spheres-sim/data/nations/laos.json) | 0.87 | 17 | — |
| [Canada](spheres-sim/data/nations/canada.json) | 596 | 13 | — |
| [Australia](spheres-sim/data/nations/australia.json) | 311 | 11 | — |
| [NewZealand](spheres-sim/data/nations/newzealand.json) | 45.5 | 24 | — |
| [DominicanRepublic](spheres-sim/data/nations/dominicanrepublic.json) | 7.07 | 32 | — |
| [Haiti](spheres-sim/data/nations/haiti.json) | 3.1 | 10 | — |
| [Jamaica](spheres-sim/data/nations/jamaica.json) | 4.59 | 14 | — |
| [TrinidadTobago](spheres-sim/data/nations/trinidadtobago.json) | 5.07 | 16 | — |
| [Bahamas](spheres-sim/data/nations/bahamas.json) | 3.17 | 30 | — |
| [Chad](spheres-sim/data/nations/chad.json) | 1.74 | 22 | — |
| [CentralAfricanRepublic](spheres-sim/data/nations/centralafricanrepublic.json) | 1.44 | 17 | — |
| [Congo](spheres-sim/data/nations/congo.json) | 2.8 | 12 | — |
| [Gabon](spheres-sim/data/nations/gabon.json) | 5.95 | 9 | — |
| [EquatorialGuinea](spheres-sim/data/nations/equatorialguinea.json) | 0.112 | 7 | — |
| [SaoTome](spheres-sim/data/nations/saotome.json) | 0.12 | 2 | — |
| [Guatemala](spheres-sim/data/nations/guatemala.json) | 7.54 | 22 | — |
| [Honduras](spheres-sim/data/nations/honduras.json) | 4.92 | 18 | — |
| [ElSalvador](spheres-sim/data/nations/elsalvador.json) | 4.82 | 14 | — |
| [Nicaragua](spheres-sim/data/nations/nicaragua.json) | 2.8 | 17 | — |
| [CostaRica](spheres-sim/data/nations/costarica.json) | 5.74 | 7 | — |
| [Panama](spheres-sim/data/nations/panama.json) | 5.24 | 12 | — |
| [Belize](spheres-sim/data/nations/belize.json) | 0.547 | 6 | — |
| [Madagascar](spheres-sim/data/nations/madagascar.json) | 3.93 | 22 | — |
| [Mauritius](spheres-sim/data/nations/mauritius.json) | 2.65 | 0 | — |
| [Seychelles](spheres-sim/data/nations/seychelles.json) | 0.369 | 0 | — |
| [Comoros](spheres-sim/data/nations/comoros.json) | 0.43 | 0 | — |
| [CapeVerde](spheres-sim/data/nations/capeverde.json) | 0.307 | 0 | — |
| [Fiji](spheres-sim/data/nations/fiji.json) | 1.337 | 5 | — |
| [SolomonIslands](spheres-sim/data/nations/solomonislands.json) | 0.215 | 10 | — |
| [Vanuatu](spheres-sim/data/nations/vanuatu.json) | 0.158 | 6 | — |
| [Samoa](spheres-sim/data/nations/samoa.json) | 0.126 | 11 | — |
| [Tonga](spheres-sim/data/nations/tonga.json) | 0.114 | 5 | — |
| [Brunei](spheres-sim/data/nations/brunei.json) | 3.52 | 4 | — |
| [PapuaNewGuinea](spheres-sim/data/nations/papuanewguinea.json) | 3.22 | 20 | — |
| [Bhutan](spheres-sim/data/nations/bhutan.json) | 0.288 | 20 | — |
| [Maldives](spheres-sim/data/nations/maldives.json) | 0.215 | 0 | — |
| [Iceland](spheres-sim/data/nations/iceland.json) | 6.695 | 9 | — |
| [Luxembourg](spheres-sim/data/nations/luxembourg.json) | 12.779 | 3 | — |
| [Malta](spheres-sim/data/nations/malta.json) | 2.547 | 3 | — |
| [Cyprus](spheres-sim/data/nations/cyprus.json) | 5.591 | 5 | — |
| [Mozambique](spheres-sim/data/nations/mozambique.json) | 3.9 | 11 | — |
| [Zambia](spheres-sim/data/nations/zambia.json) | 3.3 | 10 | — |
| [Malawi](spheres-sim/data/nations/malawi.json) | 2.7 | 28 | — |
| [Botswana](spheres-sim/data/nations/botswana.json) | 3.8 | 15 | — |
| [Lesotho](spheres-sim/data/nations/lesotho.json) | 0.6 | 10 | — |
| [Swaziland](spheres-sim/data/nations/swaziland.json) | 1.1 | 4 | — |
| [Paraguay](spheres-sim/data/nations/paraguay.json) | 4.9 | 18 | — |
| [Guyana](spheres-sim/data/nations/guyana.json) | 0.685 | 10 | — |
| [Suriname](spheres-sim/data/nations/suriname.json) | 0.577 | 10 | — |
| **Total: 137 countries** | **23459.152** | **2584** | **16 G** |

## What current provincial/sector GDP actually represents

When the optional daily province ledger is enabled, existing national GDP is allocated across owned provinces by their opening population; zero total population mass uses equal weights. The final province receives the exact remainder. Weights travel with territory. None of this measures a province's factory stock.

Every country's inherited residual uses the same game preset:

| Sector | Share |
|---|---:|
| Agriculture | 8% |
| Extraction | 6% |
| Manufacturing | 20% |
| Utilities | 4% |
| Construction | 7% |
| Transport | 10% |
| Services | 30% |
| Public services | 15% |

Evidence: `spheres-sim/src/province_economy.rs:43`, `:120–190`, and the explicit disclosure at `:489`. Current counted projects are subsequently added to their actual sectors (`:606–611`); unmapped nations retain an unallocated national account (`:638–640`).

Therefore a displayed manufacturing figure such as 20% of national GDP is **a modeled decomposition of the existing economy**, not an establishment count, production recipe, factory location, or proof that the physical layer already contains those businesses.

## Primary-source follow-up: samples, not completed verification

On 2026-09-04, the parent task queried [UNIDO INDSTAT3 metadata](https://stat.unido.org/portal/dataset/getDataset/INDSTAT/3) using the [official API contract](https://stat.unido.org/unido-statistics-portal-api). Establishments samples returned 22 two-digit rows for Japan (food/beverages 50,740; textiles 39,885) and Tonga; Tonga's aggregate footnotes prevent naive summation. Returned rows are not automatically observed numeric values. USA returned no 1990 rows for that query—not zero industry.

[WDI's 1990 manufacturing-share query](https://api.worldbank.org/v2/country/all/indicator/NV.IND.MANF.ZS?date=1990&format=json&per_page=400) returned null USA/JPN/CHN/DEU, but IND 16.5976%, GBR 16.5736%, TON 5.12585%. [WDI metadata](https://databank.worldbank.org/metadataglossary/world-development-indicators/series/NV.IND.MANF.ZS) defines value added net of intermediate consumption: neither gross output nor establishment counts.

Reproduction: resolve runtime dataset ID (149 that day); POST `/portal/dataset/getData` with `countryCode` 392/776/840, `variableCode:"01"`, `activityCodes:["15",…,"37"]`, `periods:["1990"]`, `fullPrecision:true`. Do not hardcode the runtime ID.

## Proposed architecture — not implemented

1. **Separate historical evidence from game-capacity conversion.** Author a dedicated, versioned 1990 industry inventory with stable asset/aggregate IDs, canonical country, historical boundary basis, sector classification/revision, quantity, unit, date, source, source license, confidence, and method. Distinguish verified zero, unknown, withheld/aggregated, and outside coverage. Preserve footnotes and overlapping aggregates.
2. **Do not derive “correct factory counts” from GDP.** Establishments vary enormously in size; a plant count is not a capacity standard. Record establishment count, physical output/capacity, utilization, and value added as distinct quantities where available. Conversion into the game's standard capacities must be labeled a model.
3. **Allow honest incomplete geography and sector coverage.** A national industrial total without sourced locations belongs in an unallocated national industrial account until location is established. Do not distribute plants by population and present the result as historical fact. Food, textiles, chemicals, vehicles, and other industries cannot silently become the current abstract intermediate/capital-pack recipes; retain an explicit unmodeled sector residual.
4. **Add a deterministic validated loader.** Validate all 137 canonical countries, IDs, dates, ownership, units, sources, non-overlapping aggregates, capacity bounds, and stable iteration. Missing data must remain visible rather than silently become zero or invented stock. Handle the six unmapped countries explicitly.

### No-double-count and operating requirements

- **Recognize historical output as already inside opening GDP.** Seed historical assets before `province_economy::enable`, or explicitly extend the inherited-asset registration contract. Current activation captures `gdp_projects::asset_scales`; first actual positive receipts absorb the inherited portion instead of awarding it again (`province_economy.rs:106–116`, `:330–365`; `gdp_projects.rs:544`). Installing assets afterward without this distinction looks like new output.
- **Reconcile sector baselines as well as the national total.** Current absorption reduces the general inherited residual, whose sectors still use preset shares. A historical manufacturing inventory needs a manufacturing-specific residual reconciliation, not merely unchanged total GDP.
- **Preserve actual-flow accounting.** National GDP replaces the previously included project level with current measured value added; it does not accumulate the same annual output every day (`province_economy.rs:426`). Factory receipts deduct consumed raw materials, intermediates, and purchased power; utilities receive their share once (`gdp_projects.rs:222`). Completion alone must not award production.
- **Capacity is not goods, money, sales, or construction spending.** Historical assets should not receive retrospective construction GDP, free current inventory, or treasury cash. Existing project work is separate and must not be canceled, refunded, or treated as a historical site. Avoid the old aggregate investment arm paying again for explicitly accounted production (`programs.rs:482`).
- **A seeded baseline must be operationally coherent.** Current factories need real raw/intermediate inputs, funded operating authority, delivered power/local grid, and storage. Inherited-GDP registration alone does not supply these inputs. Adding thousands of nominal capacities without a balanced, documented operating baseline could immediately stall them or double-consume flows already represented by the inherited economy. Any new baseline conversion must account for that transition, not conceal it behind a GDP bonus.

## Verification and next audit artifact

Read-only extraction for this report followed the 137 ordered embedded input paths, parsed their GDP/source notes, and reconstructed starter district ownership. It found 137 unique IDs, 2,584 distinct starting districts, zero duplicate starter ownership, 131 mapped/six unmapped nations, 16 self-origin-gap notes, and a stored GDP sum of $23,459.152bn. No whole-world simulation was run for this document.

The parent task watched this focused regression pass against current source:

```text
cargo test -p spheres-sim --release --test industry_planning all_starting_country_gdp_estimates_remain_uninvented_factory_assets -- --exact --nocapture
1 passed; 0 failed; all 137 starting nations checked inside the test.
```

That test verifies the **known missing inventory and read-model purity**, not historical accuracy (`spheres-sim/tests/industry_planning.rs:352`).

Proposed follow-up artifact: machine-readable 137-country coverage rows with source hashes, GDP/sector provenance, physical capacities by kind, module micros, pending assets, unallocated capacity, missing claims, and source-to-game conversion. Proposed regression gates: opening GDP/cash unchanged by historical decomposition; first-output absorption exactly once; sector reconciliation; no duplicate stocks/intermediate value; stable save/load; explicit unknown/unmapped coverage; and operating capacity consistent with funded/input-constrained actual receipts.

Input anchors for refreshing this table:

- Canonical roster SHA256: `BCAB3F5A03B4EC33C9DE8C783C9997021033CEC99A7679A119CB69359C188873`
- District artifact SHA256: `B6B1A88C891CCC4FA02F877CC73786D96BF81EA4269ECD8915A6759FAC480F46`
- Country GDP values: linked canonical JSON inputs above. The two hashes do not fingerprint those 137 file contents; refresh the table if country files change.

This planning report does not itself assert literal historical factory counts. The
later approved game-capacity implementation and its provenance rules are recorded
in `STARTING_INDUSTRY.md`.
