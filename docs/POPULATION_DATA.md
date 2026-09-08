# Opening population data

`spheres-sim/data/population_1990.json` freezes source observations for the 137
opening nations. It separates observations from the population game's modeled
choices. No GDP figure, population total, province population, or historical
class share is created or overwritten by this collector.

## Reproduce and verify

From the repository root, using Python 3.10+ and its standard library:

```powershell
python tools/population/fetch_social_baseline.py --check
python tools/population/fetch_social_baseline.py
```

Both commands use committed, compressed source responses and require no network.
The first verifies byte identity without writing the artifact; the second
rebuilds it. It checks the complete 137-nation roster, units, finite values,
observation years, three-way age closure and all components of each territorial
reconstruction. The ISO map is read from the existing industry collector and
checked against the actual `data/nations/*.json` IDs; names are not guessed.

To download missing responses, use `--fetch`. Updating existing source responses
requires the deliberate `--fetch --refresh` combination. This may change the
artifact because statistical agencies revise historical series. Review that diff
as a data revision, rather than silently refreshing a campaign's baseline.

The 15 gzip files in `tools/population/social_source_cache/` contain the complete
WDI responses, including missing rows and original request metadata. Compression
has a fixed zero timestamp. The generated artifact records request URLs, source
revision dates, cache paths, licenses and SHA-256 hashes of uncompressed snapshots.
Country iteration and nearest-year selection are stable, and no wall clock or
random number enters generation. These are ordinary public API requests; no
credentials or external Python packages are required.

## Sources and definitions

All numeric observations are obtained through the public [World Bank Indicators
API](https://datahelpdesk.worldbank.org/knowledgebase/articles/889392), with the
original source agency retained on every observation. The frozen API responses
report the WDI revision date `2026-07-13`; their source license is CC BY 4.0.

| Artifact field | WDI indicator | Meaning |
|---|---|---|
| `population_age_0_14` | `SP.POP.0014.TO.ZS` | Share of total population ages 0–14 |
| `population_age_15_64` | `SP.POP.1564.TO.ZS` | Share of total population ages 15–64 |
| `population_age_65_plus` | `SP.POP.65UP.TO.ZS` | Share of total population ages 65+ |
| `labor_force_participation_15_64` | `SL.TLF.ACTI.ZS` | Labor force as a share of ages 15–64 |
| `labor_force_participation_15_plus` | `SL.TLF.CACT.ZS` | Labor force as a share of ages 15+ |
| `employment_population_ratio_15_plus` | `SL.EMP.TOTL.SP.ZS` | Employed people as a share of ages 15+ |
| `unemployment` | `SL.UEM.TOTL.NE.ZS` or `SL.UEM.TOTL.ZS` | Unemployed jobseekers as a share of the labor force |
| `literacy` | `SE.ADT.LITR.ZS` | Adult literacy, ages 15+ |
| `primary_enrollment` | `SE.PRM.ENRR` | Primary gross enrollment ratio |
| `secondary_enrollment` | `SE.SEC.ENRR` | Secondary gross enrollment ratio |
| `tertiary_enrollment` | `SE.TER.ENRR` | Tertiary gross enrollment ratio |
| `education_attainment_secondary_25_plus` | `SE.SEC.CUAT.UP.ZS` | At least upper secondary completion, ages 25+ |
| `education_attainment_tertiary_25_plus` | `SE.TER.CUAT.ST.ZS` | At least short-cycle tertiary completion, ages 25+ |

Age shares originate in UN Population Division World Population Prospects and
are **midyear estimates**, used here as an opening-year approximation for the
game's January start. They are not a January 1 census. The existing nation files
remain authoritative for total population. [Age-series metadata](https://databank.worldbank.org/metadataglossary/world-development-indicators/series/SP.POP.0014.TO.ZS).

Participation and the employment ratio are ILO modeled estimates. Unemployment
prefers an actual 1990 national estimate; otherwise it uses the nearest ILO
modeled estimate, or the nearest national estimate if the modeled series has no
observation. ILO modeled unemployment begins in 1991. National definitions and
coverage can differ, and modeled observations are not all survey measurements.
These series provide opening anchors, not independently enforceable identities:
15+ and 15–64 denominators differ. The simulation must count its own workers and
jobseekers after initialization. [Participation definition](https://genderdata.worldbank.org/en/indicator/sl-tlf-acti-zs?geos=WLD&view=trend),
[unemployment metadata](https://databank.worldbank.org/metadataglossary/world-development-indicators/series/SL.UEM.TOTL.ZS).

UNESCO Institute for Statistics supplies the education series. **Attainment is
cumulative:** the upper-secondary observation includes tertiary graduates. A
model making exclusive qualification groups must subtract the tertiary group
from the secondary group. Both describe ages 25+, so assigning them to ages
15–64 is a modeling approximation. Different field observations may also come
from different years. [Upper-secondary metadata](https://databank.worldbank.org/metadataglossary/world-development-indicators/series/SE.SEC.CUAT.UP.ZS),
[tertiary metadata](https://databank.worldbank.org/metadataglossary/world-development-indicators/series/SE.TER.CUAT.ST.ZS).

Gross enrollment counts students of all ages against an official school-age
denominator. It can exceed 100%; the artifact preserves this. Enrollment is a
flow/capacity indicator, **not an adult qualification share**. Adult literacy is
also not a primary-school completion certificate. Any use of enrollment or
literacy to estimate missing qualification stocks belongs in an explicitly
modeled initialization path. [Enrollment metadata](https://databank.worldbank.org/metadataglossary/world-development-indicators/series/SE.SEC.ENRR),
[literacy metadata](https://databank.worldbank.org/metadataglossary/world-development-indicators/series/SE.ADT.LITR.ZS).

## Selection and measured coverage

Age shares require exact 1990 observations. All other fields search 1988–1992
and choose the nearest year, breaking an equal-distance tie toward the earlier
year, subject to the unemployment source preference above. Each value records
its actual year. All percentages are divided by 100; no percentage is silently
interpreted as a head count. A missing observation is JSON `null`, never zero.

| Field | Available / 137 | Exact 1990 | Territorial reconstructions included |
|---|---:|---:|---:|
| Each of the three age shares | 136 | 136 | 5 |
| Participation, ages 15–64 | 130 | 130 | 0 |
| Participation, ages 15+ | 130 | 130 | 0 |
| Employment/population, ages 15+ | 130 | 0 | 0 |
| Unemployment | 130 | 68 | 0 |
| Literacy | 43 | 14 | 0 |
| Primary enrollment | 89 | 77 | 0 |
| Secondary enrollment | 83 | 63 | 0 |
| Tertiary enrollment | 76 | 57 | 0 |
| Upper-secondary attainment, ages 25+ | 28 | 13 | 0 |
| Tertiary attainment, ages 25+ | 32 | 13 | 0 |

The JSON `coverage` section lists every missing nation per field. The collector
does not fill these gaps using richer neighbors, income rankings or invented
historical class composition. `class_shares` is explicitly null for all 137
nations. The population engine's arcade class allocation must be identified as
a game model rather than a historical census.

## Historical territories

Five opening countries need a territorial aggregate. For age shares only,
the collector weights each constituent's exact-1990 age fraction by its
exact-1990 `SP.POP.TOTL` observation:

`historical-country age share = sum(component population × component age share) / sum(component population)`

Every weight and component age fraction is recorded alongside the result, and
the quality label is `source_1990_territorial_reconstruction`. This is a
reconstruction from WDI reporter definitions, not a published historical-country
statistic. Later statistical boundary definitions and contested territories can
differ from the game's 1990 map; the reconstruction does not resolve those
differences or replace national population totals. No partial aggregate is
published if a component is missing.

| Opening country | Included statistical territories |
|---|---|
| USSR | All 15 successor republics: ARM, AZE, BLR, EST, GEO, KAZ, KGZ, LTU, LVA, MDA, RUS, TJK, TKM, UKR, UZB |
| Yugoslavia | BIH, HRV, MKD, MNE, SRB, SVN, plus Kosovo (XKX), separately reported by WDI |
| Czechoslovakia | CZE and SVK |
| Sudan | SDN and SSD, since South Sudan was part of Sudan in 1990 |
| Ethiopia | ETH and ERI, since Eritrea was part of Ethiopia in 1990 |

Their labor and education fields remain null. Those percentages need labor
force, adult or school-age denominators; averaging them with total-population
weights would give a misleading historical estimate. Russia does not stand in
for the USSR, Serbia does not stand in for Yugoslavia, and the modern Sudan and
Ethiopia series do not silently stand in for the larger opening countries.

Taiwan retains its `TWN` mapping and null observations, because these WDI
responses do not cover it. China is not used as a substitute. Germany and Yemen
use the unified-territory series, matching their single opening roster entries.

Province splits continue to use the existing GHS-POP-derived
`spheres-web/data/district_population.json`. Assigning a national age or
qualification composition across its provinces is a game initialization rule;
these sources contain no province-level age, education or class census.
