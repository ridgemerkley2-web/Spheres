# tools/population — per-district 1990 population

The mapgen pattern, verbatim: **run only when the data needs regenerating.**
The output is committed, so the game itself never needs these tools or the
source raster.

Everything here is derivation from two sourced ends. Nothing is invented:

```
LEVEL = nation.economy.population_m      already transcribed, spheres-sim/data/nations/*.json
SPLIT = share of that nation's 1990 gridded population inside the district polygon
district_population = share x population_m
```

The 2,610 shares are a **measurement** off a published CC BY raster through the
**same** Natural Earth polygons that produced the district roster, not an
authored spread. `classify_districts.py::derive_districts()` is *imported*, never
copied, and an assert fails the run if it stops reproducing `districts.json`.

## Invocation order (from the repo root)

```
python tools/population/decode_ghspop.py     # stage A — BigTIFF -> float32 memmap
python tools/population/make_population.py   # stage B — zonal pass -> the artifact
python tools/population/check.py             # LAST — verifies everything below
```

Stage A takes ~20 s and only needs re-running when the source raster changes.
Stage B takes ~4 min. `check.py --fast` skips the two slow checks (area
reconstruction and the byte-identity re-run).

## Inputs

| file | tracked? | used by |
|---|---|---|
| `spheres-web/data/GHS_POP_E1990_GLOBE_R2023A_4326_30ss_V1_0.tif` | **no** — staging data, 364 MB, re-downloadable | decode_ghspop |
| `spheres-web/data/ne_10m_admin_1.geojson` | no | make_population (district geometry, mapgen-exact ids) |
| `spheres-web/ui/index.html` | yes | make_population (the `TERRITORY` roster, via classify_districts) |
| `spheres-sim/data/districts.json` | yes | make_population, check (authoritative 2,610-id roster) |
| `spheres-sim/data/nations/*.json` | yes | make_population (`economy.population_m`, the LEVEL) |
| `tools/terrain/classify_districts.py` | yes | make_population (imported: `derive_districts`, `territory_map`, `geometry_polys`) |

Same convention as `tools/terrain`: the big raster stays untracked in
`spheres-web/data/`, and `tools/population/raster/` is a gitignored scratch cache
for the 3.7 GB decoded memmap.

### The source

**GHS-POP R2023A, epoch 1990, GLOBE, EPSG:4326, 30 arc-second.**
European Commission, Joint Research Centre. **CC BY 4.0.**
DOI [`10.2905/2FF68A52-5B5B-4A22-8F40-C41DA8332CFE`](https://doi.org/10.2905/2FF68A52-5B5B-4A22-8F40-C41DA8332CFE)

> Schiavina M., Freire S., Carioli A., MacManus K. (2023): *GHS-POP R2023A — GHS
> population grid multitemporal (1975-2030).* European Commission, JRC.

`https://jeodpp.jrc.ec.europa.eu/ftp/jrc-opendata/GHSL/GHS_POP_GLOBE_R2023A/GHS_POP_E1990_GLOBE_R2023A_4326_30ss/V1-0/GHS_POP_E1990_GLOBE_R2023A_4326_30ss_V1_0.zip`
(443 MB zip, no login). The extracted tif is sha256
`31002afc325652c7ea5b825069759689334dc282f2c136a3a038d94bc9061af2`.

**1990 is a native epoch, not a modern raster resampled backwards** — the
epochs are 1975/80/85/**1990**/95/2000…2030, and the 1990 one is built on
1990-epoch Landsat built-up surface. The decoded grid totals **5,316,175,909**
against the UN's 5.32 bn for 1990.

Alternatives were checked and rejected on the same criterion: GPWv4 R11 has no
1990 epoch (2000 onwards), WorldPop's earliest global mosaic is 2000, and every
live HYDE 3.x path is behind bot-detection with dead mirrors.

## Output (committed)

`spheres-web/data/district_population.json`, 379,944 bytes.

```jsonc
{
  "meta":    { /* provenance, licence, formula, four caveats, gap analysis */ },
  "counts":  { "RU-MOS": 8926186.4, ... },          // 2,610 — the primitive
  "nations": {
    "Russia": {
      "district_count": 86,
      "ghs_pop_1990_m": 150.264, "transcribed_population_m": null,
      "ratio_ghs_over_transcribed": null, "share_sum": 1.0,
      "districts": { "RU-MOS": { "share": 0.0594, "pop_1990": null }, ... }
    }
  }
}
```

### `nations` is keyed BY NATION THEN DISTRICT — this is not optional

`districts.json` holds **2,985 (nation, district) pairs over 2,610 unique ids**,
because a federation carries the union of its republics' districts and each
successor repeats its own subset (`spheres-sim/src/districts.rs`). **375 ids
belong to two nations.** RU-MOS is 3.09% of the USSR and 5.94% of Russia; both
are emitted. A flat `id -> share` map is ambiguous for every one of those 375,
and collapsing them drops whole nations out of the file. `check.py` step 4 exists
to catch exactly that regression.

### `counts` vs `share` — which one a consumer wants

`share` is frozen at the **January 1990 ownership grouping**. Ownership moves at
annexation, negotiated concession and federation dissolution, and the front
engine can leave half a nation occupied or encircled.

> **Any consumer needing a weight over the districts actually held THIS TICK
> must renormalise `counts` over that set** — a sum in sorted id order,
> deterministic, `O(districts held)`. `share` is a convenience for the 1990
> start only.

`pop_1990` is `share x population_m` apportioned to whole people by **largest
remainder** (ties by district id ascending), so a nation's district head counts
sum **exactly** to `round(population_m * 1e6)` — an equality `check.py` asserts,
not a tolerance.

## Why weight by population at all

Because an unweighted or area-weighted national mean **inverts the country**.
Russia, 86 districts, uniform share 1.163%:

| | share | area |
|---|---|---|
| RU-MOS (Moscow, the city) | **5.94%** = 5.1x uniform | 2,841 km² |
| RU-CHU (Chukotka) | **0.02%** = 1/53rd of uniform | 711,984 km² |
| top 10 by population | 32.8% of the people | — |
| the 10 **largest-area** districts | 8.3% of the people | — |

Not a Russia quirk — 407 of the 2,985 pairs hold >10% of their nation and 150
hold <0.1%. `check.py` step 7 pins nine such inversions as inequalities.

**This does not compete with a uniform district start; it is orthogonal to it.**
A population-weighted mean of a uniform field is exactly that uniform value
(`Σwᵢ·s = s·Σwᵢ = s`, and `Σwᵢ = 1` here to 4.4e-16 — measured at 2.1e-14 over
all 154 nations). Districts can still all start at their nation's transcribed
value, the inert proof stays trivial, and all spatial variation still emerges
from play. This file only decides *whose* deviation counts how much once play
creates deviation.

## The four caveats, all in `meta.caveats`

1. **Modelled surface.** GHS-POP disaggregates census totals onto observed
   built-up area. It is a modelled 1990 surface, not a 1990 census raster.
2. **The bias is spatial, not just scalar** — dispersed rural settlement is
   under-detected, tilting shares toward urban districts. Every nation's
   disagreement with its transcribed total is on the record as
   `ratio_ghs_over_transcribed` rather than assumed small. Median **0.988**;
   **105/131 comparable nations within ±5%**, 115 within ±10%.
3. **Modern boundaries.** The districts are Natural Earth's *modern* admin-1
   units, so 1990 people are sliced by today's internal borders — UP/Uttarakhand,
   MP/Chhattisgarh, Bihar/Jharkhand, AP/Telangana. Right for a game whose
   districts are the modern units, but a district total is not comparable to a
   1990 census line for a unit since divided; compare the sum of the parts.
   Inherited from the districts pass.
4. **Roster gaps** — below.

## Roster gaps, and the closure test that finds them

A gap is **not** "the roster omits land the nation held". It is "the transcribed
`population_m` counts people the roster has nowhere to put". Those are different,
and guessing which is which gets it wrong. So `ROSTER_GAP_CANDIDATES` are only
candidates and the **data decides**:

```
r0 = ghs_under_roster / transcribed
r1 = (ghs_under_roster + ghs_missing) / transcribed
accepted  <=>  |r1 - 1| < |r0 - 1| - 0.01
```

| nation | missing adm0 | people outside the roster | ratio | verdict |
|---|---|---|---|---|
| Sudan | SDS (South Sudan) | 4.735 m | 0.787 → 0.964 | **accepted** |
| Yugoslavia | KOS (Kosovo) | 1.917 m | 0.932 → 1.013 | **accepted** |
| Serbia | KOS (Kosovo) | 1.917 m | — (successor, no level) | **accepted** via its federation |
| USA | PRI GUM VIR ASM MNP | 3.815 m | 0.987 → 1.002 | **accepted** |
| Cyprus | CYN (N. Cyprus) | 0.235 m | 0.669 → 0.967 | **accepted** |
| Portugal | MAC (Macau) | 0.279 m | 0.975 → 1.003 | **accepted** |
| Israel | PSX (West Bank, Gaza) | 1.993 m | 1.058 → 1.486 | *rejected* |
| UK | HKG (Hong Kong) | 4.964 m | 0.995 → 1.083 | *rejected* |
| Morocco | SAH (W. Sahara) | 0.002 m | 1.007 → 1.007 | *rejected* |
| France | NCL PYF | 0.309 m | 1.002 → 1.007 | *rejected* |
| USSR | KAB (Baikonur) | 0.023 m | 1.001 → 1.001 | *rejected* |

The rejections are the point. Israel held the West Bank and Gaza in 1990 and the
UK held Hong Kong, but **their transcribed totals do not count those people**, so
"repairing" the roster would have corrupted the split. Rejected candidates stay
in `meta.roster_gap_candidates_rejected` with their numbers, so the reasoning
survives rather than living in a commit message. Two guards are asserted every
run: each named adm0 must still exist in `ne_10m_admin_1.geojson` and must still
be claimed by no nation in `TERRITORY`.

**Sudan is the one that would have hurt.** `SDN` is Natural Earth's *modern*
Sudan; multiplying its 17 districts by the transcribed 26.8 m spreads ~5.7 m
southerners across the north.

## Nations this file cannot serve

- **6 nations get no districts at all** — Bahrain, CapeVerde, Comoros, Maldives,
  Mauritius, Seychelles. Their `TERRITORY` entry claims no ISO3 code, so the
  districts pass gave them empty lists. Each one's code *is* in Natural Earth
  with admin-1 units and 1990 population (BHR 0.460 m, CPV 0.321 m, COM 0.376 m,
  MDV 0.066 m, MUS 1.083 m, SYC 0.066 m), so this is a **one-line roster
  omission, not missing data**. Measured and listed in
  `meta.nations_without_districts_codes`.
- **23 nations have shares but no `pop_1990`** — Armenia, Azerbaijan, Belarus,
  Bosnia, Croatia, EastTimor, Estonia, Georgia, Kazakhstan, Kyrgyzstan, Latvia,
  Lithuania, Macedonia, Moldova, Montenegro, Namibia, Russia, Serbia, Slovenia,
  Tajikistan, Turkmenistan, Ukraine, Uzbekistan. Successor states with no 1990
  nation file, so no `population_m` to multiply by. `pop_1990` is `null`, never
  guessed. Renormalise `counts` over the districts held.
- **13 districts have zero population** and all 13 are genuinely uninhabited:
  Macquarie, Bouvet, Antipodes, Auckland Is., Campbell, Kermadec, The Snares,
  Three Kings, Tokelau, and four Natural Earth `-99` sliver artefacts
  (COL/MEX/RUS/VEN). Not a failure. 27 more are under 1,000 people and are all
  genuinely tiny places — the smallest are BS-RC Rum Cay (15), BS-RI Ragged
  Island (63), the ARE Neutral Zone (65) and VE-W Dependencias Federales (80).

## Determinism

Sorted district order, source polygon/ring order, a supersample factor that is a
pure function of the bounding box, largest-remainder apportionment with an id
tie-break. No RNG, no wall clock, no dict-order dependence. Three runs — two to
scratch paths, one to the committed path — are byte-identical, sha256
`1f0a1884e5bf720aab3c65cedee84a6a4f7d347fdf6c86fefe24acb1a6f65547`. `check.py`
step 8 proves it mechanically by regenerating into a temp sibling (never over the
committed artifact) and comparing hashes.

## What check.py verifies

| # | check |
|---|---|
| 1 | coverage — 2,610 ids, 2,985 pairs, exactly matching `districts.json` |
| 2 | every nation's shares sum to 1 within 1e-12 (measured 4.4e-16) |
| 3 | head counts sum **exactly** to the transcribed total, 131 nations |
| 4 | the 375 dual-held ids weigh differently under each holder — the flat-map trap |
| 5 | **rasterizer, independently** — district area rebuilt from the same coverage masks with cos-latitude cell areas vs mapgen's separately computed `area_sqkm`, agreeing to 0.15–0.43% across five orders of magnitude (RU-MOS 2,851 vs 2,841 km²; CA-NU 2,072,446 vs 2,065,092 km²). The two paths share only the source polygons. |
| 6 | **real 1990 censuses**, external to every input: US-CA 29.761 m vs the 1990 US Census 29,760,021 (**ratio 1.00003**), Moscow city 8.926 m vs the 1989 Soviet census 8.967 m, Tokyo-to 11.170 m vs 11.856 m, Ontario 10.394 m vs 10.085 m, Henan 91.052 m vs 85.510 m, undivided UP 144.804 m vs 139.112 m |
| 7 | nine population-vs-area inversions pinned as inequalities |
| 8 | byte-identity of a regeneration |

## Interface note for the extraction loop and the resources swarm

Consume **`counts`**, not `share` — see *`counts` vs `share`* above. Requirements
this file places on whatever economic model wins:

- It must accept a **weight per district**, not a per-district authored value.
  This file supplies weights and a 1990 level; it does not supply per-district
  economic state and must not be read as if it did.
- It must renormalise over **districts held this tick**, in sorted id order.
- It must tolerate `pop_1990 == null` for the 23 successor states, and
  **no districts at all** for the 6 unserved nations.
