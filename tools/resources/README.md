# tools/resources — 1990 resource transcription

The mapgen pattern, verbatim: **run only when the data needs regenerating.** The
output is committed, so the game never needs these tools or the source archives.
Everything here is transcription — every deposit exists because a public-domain
dataset put a named site at a coordinate that falls inside that district's
polygon. Nothing is interpolated, scattered, scored, or ranked.

## Invocation order (from the repo root)

```
python tools/resources/make_resources.py
python tools/resources/check_resources.py
python tools/resources/check.py
python tools/resources/make_resources_1990.py
python tools/resources/check_resources_1990.py
```

`make_resources_1990.py` writes the sim's table, `spheres-sim/data/resources_1990.json`:
the 1990 national figures keyed by roster code, a location weight per district
for each producer (the share of the 1990 owner's figure that sits there, by one
written rule, pruned below 1e-3 and renormalised), presence bits, a presence
rank, the 1990 population shares, and the one new transcription — a 1990 unit
value per mined line with its source. `check_resources_1990.py` holds that file
against the inputs on disk, the roster, and byte-identical regeneration.

`check_resources.py` verifies the committed artifact against its own declared
rules: roster membership, provenance completeness, the confidence grading, the
named 1990 mining and petroleum regions, the fabrication guards, the
contamination the 2026-08-31 pass removed, the petroleum apportionment and its
boundary-ribbon floor, the coal coverage against the 1990 producers, and
byte-identical regeneration. 169 checks.

Two of its groups exist to defend ruling 4 and the coal pass specifically, and
both were added after those passes shipped, so they test the result rather than
restating the intent:

- **3b, banding on distinct sites.** The band is recomputed here from
  `confidence.distinct_coordinates` against a threshold table written out
  longhand, so a generator that bands on the wrong quantity fails instead of
  agreeing with itself. Beside it sits the check the whole ruling depends on:
  **no published coordinate moved.** Every cited coordinate is held against the
  one its source published, to five decimals, re-read from `mrds-csv.zip` — a
  silently corrected point would otherwise change a band with nothing to catch
  it. The group also pins the nine named cases from the defect report, asserts
  that the site metric can only ever *shrink* an evidence claim, and asserts the
  FRA defect was closed by counting rather than by deletion (all six records
  still ship, on the published point, all flagged).
- **3c, the named coal regions.** The Ruhr, Upper Silesia, the Donbas,
  the Kuzbass, Karaganda, Ekibastuz, the Pechora, Shanxi, Appalachia and the
  British pits, pinned by district id and by the *shape* of their evidence — a
  named mine or a named field — so a district that acquires coal from the wrong
  source fails rather than merely being present. The Ruhr and Upper Silesia are
  asserted as the two ends of the ruling: the same source and the same year, one
  earning `strong` on ten real pitheads, the other reduced to `single` because
  its seven rollup rows share one point. The group also re-asserts both coal
  defects — the per-named-field aggregation of Appalachia, and the `china2014`
  per-province `IDNum` key — and that no coal attachment carries a tonnage.
  Finally it re-derives the coal admission arithmetic from raw `minfac.csv`
  using the artifact's own published vocabulary — **380 candidate rows, 21
  removed, 359 admitted** — and requires the Ruhr's pitside coking plants to be
  among the removed. Take that candidate set from the vocabulary and never by
  searching for the substring `coal`: the bare `Coke` and `coke` rows do not
  contain it, and a substring filter silently misses them and under-reports the
  guard (373/14 instead of 380/21).

Each of these was verified to go red: injecting a moved coordinate, a
record-banded entry, a de-aggregated Appalachia, a tonnage on a coal field, and
a "fix" that deleted Silesia's rollup rows raised nine failures between them.

`check.py` runs LAST and is the **ground-truth** pass — deliberately a second,
independent opinion rather than more of the same. It imports neither
`sources.py` nor `make_resources.py` for the admission rules: it re-reads all
304,632 raw `mrds.csv` records and rebuilds the admitted set from the CSV
columns, so a bug shared between the generator and its own checker cannot hide
in both. It does the same for coal against raw `minfac.csv`. 96 checks, in eight
groups:

| group | what it holds the artifact against |
|---|---|
| 1. structure | roster membership, the artifact's own coverage counters, and the rule that **no mineral entry carries a magnitude key of any kind** |
| 2. the plant guard | every one of the 5,018 cited MRDS sites re-derived from raw CSV: Producer/Past Producer, **not** Processing Plant or Geothermal, commodity named in `commod1`/`commod2` and never only `commod3` |
| 2b. the coke guard | every cited coal site re-derived from raw `minfac.csv`: no coke plant admitted as a mine, no `fac_type = Plant` admitted, and every coordinate still the one minfac published |
| 3. positive ground truth | the Persian Gulf, the Copperbelt and Katanga, Antofagasta, the Pilbara and Weipa, the Witwatersrand and the Bushveld, Krivoy Rog and the Urals, Jamaica, Khouribga — and, since the coal pass, the Ruhr, Upper Silesia, the Donbas, Appalachia, Shanxi, the British pits, the Kuzbass, Karaganda, Ekibastuz and the Pechora. Pinned by district id, each reporting its measured band |
| 4. negative ground truth | no bauxite in Norway or Japan, nothing at all in Switzerland or Singapore, no coke plant filed as a coal mine, no US petroleum |
| 5. apportionment | the ruling-2 arithmetic, verified to the artifact's own published precision (see below), plus the Iraq / Nigeria / Mexico rankings |
| 6. the honest-limit ledger | the single-evidence share, the continental coverage spread, the MRDS coordinate collisions, the Guinea hole, and the three structural holes |
| 7. determinism | regenerate, require the SHA-256 unchanged |

Pass `--fast` to either script to skip the regeneration check.

**On tolerances.** `check.py` never widens one to get green. The apportionment
identity is checked against a bound derived from the artifact's own published
precision — `known · 5e-7 + 5e-4`, being a half-ulp of the 6-dp
`area_frac_province` times the province total plus a half-ulp of the 3-dp
`apportioned` — and the measured worst case is **0.9946 of that bound**. Every
value fits under it; none is merely inside a generous margin. The same reasoning
sets the per-province sum bound at `known + 5e-4 · contributors`, where the
worst measured excess is +0.0020 on a 30,731-unit province with 30 contributors.

**Failures versus warnings.** `check.py` exits non-zero only on a FAIL. The nine
WARNs are not regressions — they are the shape of the sources, and they are
printed every run precisely so they cannot quietly stop being true. A WARN that
disappears is as much a signal as a FAIL that appears.

All three take about 25 seconds each. None needs a network or a build. The one
non-standard-library dependency is **shapely** (tested on 2.1.2), which does the
petroleum-province polygon clipping and the STRtree; everything else is stdlib.

## The 2026-08-31 correction pass

The first edition shipped two defects that two independent audits found:

- **The metal was mapped onto the ore.** MRDS's commodity vocabulary has no
  `Bauxite` token at all. `Aluminum` — the refined metal — was mapped straight
  onto `bauxite`, and MRDS uses that token at bauxite mines and at aluminium
  smelters alike. Norway's nine Aluminum records are nine smelters and not one
  mine, so Norway shipped as a bauxite producer; so did Japan, on ten.
- **Assay was read as production.** USGS defines `commod3` as commodities
  "economically interesting but **not economically recoverable**". Steep Rock
  Iron Mine (MRDS 10157857, Ontario) has an empty `commod1` and `commod2` and a
  `commod3` reading *Aluminum, Iron, Manganese, Silica, Sulfur,
  Phosphorus-Phosphates*, so it filed as a bauxite deposit, an iron deposit and
  a phosphate deposit at once.

Four admission rules now stand between an MRDS row and a district entry. Every
value of every controlling vocabulary is enumerated with a decision and a
reason — USGS's own field definitions, quoted — in `sources.py`, and shipped in
the artifact at `meta.mrds_vocabulary`. **A value present in the data and absent
from the decision table raises**, so a new MRDS edition cannot slip a new
operation type past the filter by defaulting.

| rule | field | rejected | why |
|---|---|---|---|
| 1 development status | `dev_stat` | Occurrence, Prospect, Unknown, **Plant** | no production asserted; `Plant` is USGS's own smelter marker |
| 2 operation type | `oper_type` | **Processing Plant**, **Geothermal** | "No ore extraction at the site, only a mill, smelter, etc."; "energy extracted from heat stored in the earth" |
| 3 commodity tier | `commod1..3` | **commod3** | USGS: "not economically recoverable" — assay, not output |
| 4 ore stage | `ore`, `dep_type` | non-bauxite aluminous ore minerals | only `bauxite` needs it; see below |

Rules 2, 3 and 4 are new. What each removed, per commodity and per nation, is
in the artifact at `meta.correction_2026_08_31.removed`.

### The ore-versus-refined-metal audit

Every game commodity was checked against MRDS's 183-token vocabulary, not just
the one that was known to be broken. `iron`, `copper`, `gold`, `uranium` and
`phosphate` all take ore-stage tokens — MRDS carries `Pig Iron`, `Copper Oxide`
and `Sulfuric Acid` as separate tokens for the processed forms — and the three
PP1802 commodities come from a deposit catalogue with no plants in it. Only
`bauxite` was wrong, and it was wrong because MRDS has no word for the ore.

The general trap, recorded because it nearly caused a second error: MRDS's
refined-stage tokens (`Pig Iron`, `Contained or Metal`, `Smelter`, `Refinery`,
`Ferrochrome`, `Mill Concentrate`) are **form qualifiers attached to a
commodity, not site classifications**. All 43 producing `Pig Iron` records sit
at named magnetite and gossan mines and none is a plant. Filtering on them
would have deleted real iron mines. `oper_type` is the field that classifies
the site, and that is what rule 2 uses.

### Bauxite, and the absence that was not created

Rule 4 rejects an `Aluminum` extraction record only where the source's own `ore`
column names a different aluminous mineral — alunite, kaolinite, corundum,
dumortierite. It **admits on silence**: 511 of the 772 surviving records have a
blank `ore` column, and demanding positive proof would have erased China, India,
Hungary, Kazakhstan, Greece and Suriname from the bauxite map. A fabricated
absence is exactly as dishonest as Norway's fabricated presence.

It also refuses to infer from a co-listed commodity. Georgia's Eufaula,
Andersonville and Irwinton bauxite districts and Australia's Weipa Andoom are
all filed with `Kaolin` beside `Aluminum`, because bauxite genuinely occurs with
kaolin. A co-commodity heuristic would have deleted four real bauxite districts
to remove none.

## The 2026-08-31 coal pass

The first two editions shipped this sentence, and it was wrong:

> No location source exists for coal at district resolution: MRDS holds 157 coal
> records for the entire world.

The first half does not follow from the second. What had been established was
that **MRDS** does not carry coal. What was printed was that **nothing** does.
The cost of the difference was the whole 1990 coal map: Nordrhein-Westfalen
carried copper, gas, gold and oil and no coal while Germany's national figure
read 434 million tonnes; 59 nations were national-only, including China at
1,079,300 kt, the USA at 933,562 kt and the USSR at 710,999 kt. For a game that
opens in January 1990, Silesia, the Donbas, Appalachia, Shanxi, the Ruhr and the
aftermath of the miners' strike are the politics, and the map said none of it
existed.

USGS publishes coal locations. It publishes them somewhere other than MRDS.

### The four sources, and why it takes four

| key | dataset | what it gives | licence, as its own metadata states it |
|---|---|---|---|
| `minfac` | [Mineral Operations Outside the United States](https://mrdata.usgs.gov/mineral-operations/) — the Minerals Yearbook facility series (OFR 2006-1135, 2006-1375, 2010-1254, 2010-1255, 2010-1257) | **359 admitted coal mines** (358 placed; one falls in no district), world ex-US, 58 nations, with name, status and reporting year | `Access_Constraints: none` / `Use_Constraints: none` |
| `china2014` | [USGS OFR 2014-1219](https://pubs.usgs.gov/of/2014/1219/) `AllChinaCoalMines` | **2,440 named Chinese mines** with province, county, rank and size class — 242 in Shanxi | `accconst: none` / `useconst: none; interpretations must acknowledge USGS as source` |
| `fsucoal` | [USGS OFR 01-104](https://pubs.usgs.gov/of/2001/ofr-01-104/fsucoal/html/data1.htm) `deposit.shp` | **163 named coal deposit and basin polygons** of the FSU — Donetsky, Kuznetsky, Karagandinsky, Ekibastuzsky, Pechorsky, L'vov-Volynsky | `Access_Constraints: None` / `Use_Constraints: None` on the coal layer |
| `uscoalfields` | [USGS OFR 2012-1205](https://pubs.usgs.gov/of/2012/1205/) `Coal_Fields.shp` | **602 coal field polygons** of the conterminous US in 110 named fields, 208 polygons of them "Appalachian Region" | `accconst: None`; the use statement is a warranty disclaimer, not a restriction |

They are complementary by construction and none is a subset of another. minfac
is *explicitly* "outside the United States"; uscoalfields is exactly the United
States. china2014 carries the mines minfac reduces to eleven province rollups —
"Mine in Shanxi" at 37N 112E is one row. fsucoal carries the Soviet basins as
the outlines they are.

**On the ESRI clause in OFR 01-104.** That report's readme restricts
redistribution of *"the coastline and country boundaries"*, which are ESRI
property used with permission. Those are the `cis`, `roads`, `rail`, `rivers`
and `lakes` layers. **None of them is read here.** The coal layer is
USGS/Vernadsky work and its own FGDC metadata states no constraint; that is the
only layer this pipeline touches.

### The coke trap

It is the bauxite trap again, and it was waiting in the same place.

Coal has a processed form with a name of its own — **coke** — and the Minerals
Yearbook files coke ovens in the same table as coal mines, at the same kind of
coordinate, under a commodity string that contains the word *coal*:

```
Coke: contained in domestic coal
```

That is a coke plant's throughput expressed in the coal it ate. Five of them sit
in the Ruhr, one of them at Bottrop — where Prosper-Haniel already is. Admitting
them would have put the Ruhr on the map for the wrong reason and counted Bottrop
twice.

Two rules stand in the way, both written against **the source's own
classification fields**, and every value of both vocabularies is enumerated in
`sources.py` with a decision and a reason, and shipped at
`meta.coal.minfac_vocabulary`:

| rule | field | rejected | why |
|---|---|---|---|
| A facility type | `fac_type` | **`Plant`** | USGS's own marker: the type of operation is a processing plant, so no coal is extracted at the coordinate. Blank is **admitted** — silence is not a denial, the same reading that admits MRDS's `Unknown` oper_type |
| B commodity stage | `commodity` | **`Coke`, `coke`, `coal - coke`, `Coke: contained in domestic coal`** | coke is coal carbonised in an oven: a manufactured fuel, not a thing mined |

Rule B is matched on the substring *deliberately*: every commodity string
containing "coal" **or** "coke" is routed through the decision table, which is
how the coke rows are made to reach a decision rather than be missed. A value
present in the data and absent from the table **raises**, so a later Yearbook
edition cannot slip a new oven past the filter by defaulting.

The rules removed 21 of 380 candidate rows — 14 to rule B, 7 to rule A — and the
counts are in the artifact at `meta.coal.removed`. `check.py` re-derives the
whole thing from raw `minfac.csv` without importing `sources.py`, and asserts
that none of the 14 coke rows and none of the 20 `Plant` rows is cited.

### A named field is not a polygon

OFR 2012-1205 draws the Appalachian Region as 208 separate polygons, 102 of
which clip Pennsylvania. Banding on the largest single fragment would have read
Pennsylvania as `moderate` on 26% coverage; the fragments are a partition of one
mapped region and do not overlap, and together they cover **30.4%**. So the
attachment is aggregated **per named field**: intersections summed, the polygon
count published beside the sum so the transcription is still visible,
`area_frac_field` measured against that name's whole mapped area. West Virginia
reads `strong` on **69.6%** of itself inside the Appalachian Region.

This is ruling 4's mistake in a new place, and it was caught before it shipped
by asking the same question ruling 4 asks: *what is the band actually
counting?*

### What the checker caught that the generator did not

`china2014`'s `IDNum` **restarts at 1 in every province**. It is the number on
the province-scale map sheet the report publishes, not a national key: 2,440
mines carry only 253 distinct values, and Shanxi alone runs 1 to 242. Treating
it as an identifier collapsed 2,440 mines into 253 pieces of evidence — and it
did, until `check.py`'s independent recount found an entry reporting **152
records on 153 distinct coordinates**, which is impossible. The source's own key
is the pair, so `province#IDNum` is the id. This is precisely why `check.py`
recomputes rather than reads.

### Two kinds of evidence, both published, the stronger taken

Coal is the only commodity placed from more than one dataset, and the only one
that can arrive on a district as two independent kinds of evidence at once:
named mines from a point source, and a named coal field from a polygon source.
Donetsk has both.

Where that happens the entry keeps **both** measurements, states **both** bands,
and takes the stronger. Two independent surveys agreeing is not weaker than
either alone, and averaging them would invent a grade neither supports. The band
basis is stated per entry:

| `confidence.banded_on` | means |
|---|---|
| `distinct_coordinates` | mines only — 6+ strong, 3-5 moderate, 2 sparse, 1 single |
| `coal_field_area_frac` | coal field only — 50%+ strong, 15-50% moderate, 2-15% sparse, under 2% single |
| `stronger_of_points_and_fields` | both, with `band_on_points` and `band_on_fields` beside it |

`check.py` recomputes all three independently.

### One thing that had to be measured rather than assumed

Coal's point records join the shared coordinate census, because the census
exists to find filing centroids and coal is the most centroid-ridden source in
the file — the eight "Mine at Upper Silesia" rows on one point are exactly what
it is for. That means 2,798 new records passed through a rule that is evaluated
across commodities: the census went from 60,321 records on 56,694 coordinates to
63,119 on 59,307, and the flagged coordinates from 492 to 531.

Whether any of those 39 new flags landed on a coordinate an existing entry
stands on is a question, not an assumption, so it was measured. Re-running the
generator with coal excluded from the census **alone** and diffing the result
entry by entry: **zero non-coal entries change.** All 39 are on coordinates only
coal occupies. Nothing that shipped before this pass moved because coal arrived,
and the measurement is recorded in the artifact at
`meta.coal.coordinate_census_effect` rather than asserted here only.

### Coverage — what the coal hole looks like now

**328 districts in 63 nations carry coal.** Of the **59 nations that produced
coal in 1990, 45 now have it located**; 14 do not, and they are named below.

| region the brief named | district | band | the evidence, from the source's own words |
|---|---|---|---|
| **the Ruhr** | `DE-NW` | `strong` | 10 named collieries — Prosper-Haniel, Walsum, Lippe, Auguste Victoria/Blumenthal, Lohberg-Osterfeld, Ibbenbüren, and the Rhenish lignite pits Garzweiler, Hambach, Inden, Bergheim |
| **Upper Silesia** | `PL-SL` | `single`, flagged | 7 minfac rows — *"Mine at Upper Silesia (17 mines)"* and seven more — **all on one coordinate**, 50.17N 18.83E. One point is one site's worth of evidence, and the filing-centroid flag says so |
| **the Donbas** | `UA-14` Donets'k | `moderate` | OFR 01-104's `Donetsky basin` polygon covering **39.1%** of the district, plus minfac's "Donets Basin" row |
| | `UA-09` Luhans'k | `moderate` | the same basin covering **31.6%** |
| **Appalachia** | `US-WV` | `strong` | OFR 2012-1205's `Appalachian Region` covering **69.6%** of West Virginia across 14 mapped polygons |
| | `US-PA` | `moderate` | the same region, **30.4%**, 102 polygons |
| | `US-KY` | `moderate` | **24.1%** |
| **Shanxi** | `CN-SX` | `strong` | **169 distinct mine coordinates from 243 records.** minfac alone would have given Shanxi one province centroid at 37N 112E |

And the ones the brief did not name but 1990 turns on:

| | district | band | evidence |
|---|---|---|---|
| British coal after the strike | 8 GB districts | up to `strong` | Kellingley, Maltby, Thoresby, Welbeck, Daw Mill, Tower Colliery and the Scottish and Welsh opencast pits — **the twenty left in 2007**, not the pits the strike failed to save |
| the Kuzbass | `RU-KEM` | `moderate` | `Kuznetsky basin`, 33.5% |
| Karaganda | `KZ-KAR` | — | `Karagandinsky basin` |
| Ekibastuz | `KZ-PAV` | — | `Ekibastuzsky basin` |
| the Pechora | `RU-KO` | — | `Pechorsky basin` |

Every one of the ten largest 1990 producers now has located coal: China, the
USA, the USSR, Germany, India, Poland, Australia, South Africa, Czechoslovakia
and the United Kingdom.

### What it still cannot say, and the shape of what is left

**None of the four sources is a 1990 census.** minfac is 2003-2008 and stamps
the reporting year on every site; china2014 digitises a 2001 atlas; fsucoal was
published in 2001; uscoalfields in 2012. The gap is one-directional and it
**understates** 1990 coal — it does not invent any — but it bites hardest
exactly where 1990 politics was hottest. The Ruhr appears as the nine collieries
still open in 2006, not the twenty-odd working in 1990. British coal appears as
the twenty pits left in 2007.

That is visible in the 14 producers still unlocated, which are almost entirely
the mines that closed in between:

| nation | 1990 output (kt) | why |
|---|---|---|
| France | 12,820 | the last French pit closed in 2004 |
| Austria | 2,448 | last lignite mine closed 2005 |
| Philippines | 1,243 | no minfac coal row |
| Belgium | 1,036 | Zolder, the last colliery, closed 1992 |
| Italy | 956 | Sulcis wound down through the nineties |
| Portugal, Zaire, Malaysia, Peru, Malawi, Ireland, Sweden, Algeria, Bhutan | 281 and below | small or closed producers |

Every one of them is marked in `unlocated_producers.coal` with the reason. None
is filled with a guess.

**A coal field is not a mine.** OFR 2012-1205 states outright that it "does not
differentiate between potentially minable coal and uneconomic coal". A district
overlapping the Appalachian Region polygon has coal *under* it; whether it mined
any in 1990 is a different claim and this artifact does not make it. That is why
a field entry publishes an **area** fraction and never a tonnage, why the band
on fields is reported separately from the band on mines, and why the browser
layer says "coal-bearing GROUND, not output" on the hover line.

**There is still no district tonnage.** minfac carries a `capacity` column and
it is *deliberately not transcribed*: it is a 2003-2008 facility capacity, and
reading it as 1990 output — or summing it per district — is the exact
fabrication the apportionment note forbids. China's `MineSize` is a three-value
class (Large / Medium / Small) and ships as a band label for the same reason: it
is a word, not a number.

**Never count.** `n` is a citation count, and one minfac row can be seventeen
Silesian mines.

### Sources probed and rejected

Both are recorded in the artifact at `sources_rejected`, because a documented
dead end is a result.

- **Global Energy Monitor, Global Coal Mine Tracker** — reachable (HTTP 200),
  genuinely open (CC BY 4.0), ~7,000 mines in 70 countries, and rejected twice
  over. First, the download is gated: there is no direct file URL, only a form
  served by `api.globalenergymonitor.org` that requires an email address, and
  this pipeline does not submit forms or hand over an address on the owner's
  behalf. Second — and decisive even if it were ungated — the tracker's own
  scope is *"mines abandoned or permanently closed since 2015"*. A 1990 map
  needs the pits that closed **before** 2015: most of the Ruhr, almost every
  British colliery, much of the Donbas. It is the wrong vintage by
  construction. Worth buying or requesting for a present-day map. Not for
  January 1990.
- **USGS World Coal Quality Inventory, OFR 2010-1196** — reachable, public
  domain, global, with decimal lat/lon and a location-accuracy estimate on
  every row, and still not used. It is a coal **quality sample** inventory:
  1,580 analyses collected 1995-2007, 1,538 with a coordinate. A sample is not
  a site — the UK's 84 rows collapse to 22 coordinates, Norway's 28 to one, and
  South Africa's 40 carry no coordinate at all. It has **no Germany, no Poland
  and no United States**, so it reaches none of the three holes that mattered.
  Its only published form is a 2003-vintage binary `.xls` that no
  standard-library module reads. Its one real advantage is **Turkey** (143
  coordinates), which minfac lacks entirely — recorded so that gap is a known
  trade and not an oversight.
- **MSHA Mines Data Set** — not fetched. It would give mine-level US coal with
  coordinates and an abandoned/active status, finer than the coal-field
  polygons used instead. Named as the obvious next improvement to United States
  coal resolution, not as a dead end.

## The rule that shapes the artifact

**`where` and `how much` come from different sources and are never multiplied
together.**

| | source | lands on |
|---|---|---|
| WHERE | MRDS + PP1802 deposit points, WEP province polygons | **districts** |
| HOW MUCH | EIA, USGS DS896, USDA PSD 1990 production | **nations** |

The temptation is to spread a country's 1990 copper across its copper districts
by deposit count. That would be a fabrication with a citation stapled to it. As
the source probe established, **MRDS record density measures how hard the USGS
looked, not what is in the ground** — Canada carries 147 cobalt records to
Zaire's 37, though Zaire was more than half the world's cobalt in 1990. In the
finished artifact the skew is stark and measurable: 96.6% of uranium records,
87.5% of iron records and 87.3% of gold records sit in the United States.

So districts carry **presence and evidence**; nations carry **tonnage**; the
join is left to the consumer, who gets both halves and a written warning. The
artifact's own `meta.do_not` says it, and `check_resources.py` asserts that no
district entry carries a tonnage, grade, production or share field at all.

The petroleum provinces are the one place a derived number is written down, and
it is fenced. A WEP province covers many districts and carries a real assessed
volume, so each district records the province it lies in **and that province's
whole figure, flagged `shared: true`** — the checker verifies every one still
equals its source record byte-for-byte. Beside it, and clearly marked derived,
sits `apportioned`: that volume times the measured share of the province polygon
lying in this district, which is what ruling 2 asked for. The 1990 national
production figures are not touched by any of it.

## Petroleum provinces: the real intersection, and the apportionment on it

The first edition asked a 0.25° lattice "does this province touch this
district" and recorded a yes as an entry indistinguishable from any other. That
is how **Al-Anbar** came out carrying the same four-province oil entry as
**Al-Basrah**, headlined by the whole 292,442 MMBO Mesopotamian Foredeep — in
the Iraq–Kuwait scenario this game is most known for. The second edition
measured area by latitude-row integration, which fixed the ranking but
discretised latitude and could not resolve a sliver.

This one does the actual thing. `geo.ProvinceIntersector` owns it:

1. **Geometry.** Province rings arrive from the shapefile as a flat ESRI list —
   clockwise exterior, counter-clockwise hole, no part index.
   `rings_to_geometry` classifies each ring by its shoelace sign and rebuilds
   the true Polygon/MultiPolygon, so a multi-part province is a multipolygon and
   a hole is a hole. (Reading ring 0 as the exterior and every later ring as its
   hole — which the flat list invites — is wrong the moment a province has two
   parts.) Everything goes through `make_valid`, because a self-touching
   digitised outline makes GEOS refuse the clip outright.
2. **Intersection.** An **STRtree** over the district geometries finds
   candidates; `province.intersection(district)` in WGS84 lon/lat does the rest.
   Exact polygon clipping, not sampling.
3. **Area.** Green's theorem on the sphere: for a region bounded by straight
   lon/lat edges, area = −∮ sin φ dλ, and each edge integrates in closed form.
   **Exact** for the polygon as the source stores it — no rows, no midpoint
   rule, no discretisation constant to cancel. Results are km² on a 6371.0088 km
   sphere, so they can be held against `districts.json`'s own `area_sqkm`, and
   the checker does exactly that on 1,377 attachments.

The four edge cases, all real in this data: a province spanning many districts
(the Mesopotamian Foredeep reaches 22 across three nations), a district touched
by several provinces (Al-Anbar meets seven), a province extending beyond every
district (the Niger Delta is 82% offshore), and multipolygons with holes (Tian
Shan Foldbelt has three, East Greenland one; districts are routinely
multipolygons and a district's parts are unioned before clipping so two touching
parts cannot double-count).

Each attachment carries four numbers.

| field | means | kind |
|---|---|---|
| `intersection_sqkm` | the clipped area | measured |
| `area_frac_district` | how much of this district the province covers | measured |
| `area_frac_province` | how much of the province lies in this district | measured |
| `apportioned` | `known` × `area_frac_province` | **derived** |

`known` is untouched and still flagged `shared: true`, and the checker still
holds every one against the WEP record byte-for-byte. `apportioned` is ruling
2's apportionment and it is labelled derived everywhere it appears — on the
attachment, on the district entry's `apportionment` block, and in
`meta.apportionment`. It is an area weighting and nothing more: oil is not
distributed evenly through a sedimentary basin. **1990 production is a different
thing entirely** — national, in the `national` block, and still never divided.

The weight is the share of the **whole** province, not of its on-land part. A
province 83% offshore apportions 17% of its volume and leaves the rest in
`provinces[].unapportioned_offshore`. Normalising over the on-land part instead
would shove all of Cantarell and all of Ekofisk onto the nearest coast — the
same fabrication as before, wearing an equation. Across all attached provinces
373,476 of 1,398,059 assessed MMBO (26.7%) sits unapportioned offshore.

### The boundary ribbons, and the three that would have given the USA oil

Exact clipping finds overlaps sampling could not, and some of them are not real.
WEP's North American polygons are cut at the 49th parallel and the Yukon–Alaska
line, and those cuts miss Natural Earth's border by a few hundred metres. The
result: **0.14 km² of the Alberta Basin in Montana, 0.08 km² of the Williston
Basin in North Dakota, 13.9 km² of the Mackenzie Foldbelt in Alaska.** Kept,
they hand the world's #2 oil producer three located oil districts made entirely
of a digitising mismatch.

An attachment is dropped when it is negligible in **both** polygons — under
1e-4 of the district *and* under 1e-4 of the province. Requiring both is what
keeps the genuinely tiny districts: Port of Spain is 0.77 km² of the East
Venezuela Basin, but that is 5.3% of Port of Spain. Thirteen are dropped, every
one a cross-border ribbon, and all thirteen are listed with their measurements
at `meta.boundary_slivers` — the floor is **a stated choice, not a discovered
constant**, and it is put on the record so it can be re-argued. Two districts
(Oost-Vlaanderen, Sjælland) leave the map because a ribbon was all they had.

### Offshore provinces, and why Norway has no oil

Summing `area_frac_province` over every district a province reaches says how
much of the province is on land in the roster at all — and four provinces come
back at 0.09%, 0.22%, 0.26% and 0.35% (Vestfjord-Helgeland, the Lesser Antilles
Deformed Belt, the North Sea Graben, the Santos Basin), against 1.15% for the
next one up. Those are wholly offshore provinces whose polygons clip a
coastline.

Such an attachment is measured, so it is kept and flagged
`offshore_or_outside_roster`, and its district entry is banded `single` — the
weakest mark on the map — because Norway's 1.63 mbd and the UK's 1.82 mbd in
1990 came out of the water, not out of Nordland. The 1% cut is **a stated choice
placed in a measured gap, not a discovered constant**; the full distribution
ships in the artifact's `provinces` block so it can be re-argued against the
numbers.

Provinces WEP carries with a zero known volume or its `-9999` "not assessed"
sentinel are listed there too rather than dropped, because that silence is
load-bearing: `Campeche-Sigsbee Salt Basin` is one of them, which is why no
measurement here can make Campeche Mexico's leading oil district. Mexico's Bay
of Campeche oil is not missing from the artifact — it is the 19,534 MMBO of the
Villahermosa Uplift lying outside every district, sized and named as offshore,
against a Mexico whose leading **land** district is Tabasco.

## Confidence — the map states its own confidence

Every district-commodity entry carries `confidence.band`: `single`, `sparse`,
`moderate` or `strong`. Single-site districts are **kept and rendered weaker**
rather than filtered out. A graded map is more honest than a sparse one and more
honest than a uniform one that makes an n=1 record look like the Copperbelt.

The two bases are different measurements and are labelled as such, because
banding a province by its record count would be meaningless — one polygon
covering the whole of Al-Basrah is stronger evidence than four clipping the
corners of Al-Anbar.

| basis | strong | moderate | sparse | single |
|---|---|---|---|---|
| `mrds_records` / `pp1802_deposits` | 6+ distinct **coordinates** | 3–5 | 2 | 1 |
| `province_overlap` | ≥50% of the district | ≥15% | ≥2% | >0, or offshore-only |

### Ruling 4 — the band counts sites, not records

The first edition banded a point-sourced entry on its **record** count, and MRDS
stacks records on administrative centroids. `46.56346,2.55405` — the centre of
France — carries **27 admitted records** under three commodities. Six of them
are the six Var bauxite mines, and they made `FRA_centre-val-de-loire` read
`strong` on evidence from a single fictitious point 400 km from the Var. A band
that one coordinate can carry to `strong` does not mean what a reader takes it
to mean.

**The correction is arithmetic, not editorial. No coordinate was moved,
corrected or dropped.** Every record still ships at the coordinate its source
published; `n` still counts records and `confidence.records` still counts
distinct record ids. What changed is what the band **counts**:
`confidence.distinct_coordinates`, because six records at one point is one
site's worth of evidence whatever the six are called. Thresholds are unchanged
so the two editions compare directly, and every entry carries
`confidence.superseded_band_on_records` so a reader can see what it used to say.

Measured against the previous edition, over 1,581 point-sourced entries:

| band | was (records) | now (coordinates) | delta |
|---|---|---|---|
| `strong` | 411 | **383** | −28 |
| `moderate` | 284 | **261** | −23 |
| `sparse` | 227 | **229** | +2 |
| `single` | 659 | **708** | +49 |

93 entries changed band and **every one of them fell** — the metric can shrink
an evidence claim, never inflate one. The four hardest falls are the four
national centroids: `FRA_centre-val-de-loire` bauxite (6 records, 1 coordinate),
`ESP_madrid` iron (6, 1), `ITA_umbria` copper (8, 1) and `SE-Y` copper (14, 1),
all `strong` → `single`. `AU-QLD` bauxite fell `strong` → `moderate`: four of
its six Weipa records are filed at the centroid of Queensland, 1,300 km from
Cape York, and only two coordinates in Cape York are real evidence.

### The filing-centroid flag

`confidence.centroid_stacked` is true when any coordinate under the entry is a
**filing centroid** — a point where records were filed, not a place where a mine
is. Two clauses, either sufficient:

| clause | test | why it is not enough alone |
|---|---|---|
| **(S)** `stacked_names` | 3+ **distinctly named** sites share the one published coordinate | misses a lone national rollup: one bauxite record named `Bauxite - Vietnam` on Vietnam's centroid |
| **(A)** `admin_centre` | within **2 km** of the centroid of a Natural Earth 10m admin-0 or admin-1 polygon, whole geometry or largest part | misses the centre of France: Natural Earth's France includes Guyane and Réunion, and its mainland centroid is 10 km from the point MRDS used |

Distinct **names**, not records, so a mine reported once per production year
(Homestake 1944, 1945, 1946) is a duplicate and not a stack. Both centroids are
computed per unit and the nearer wins, because Italy is clause (A)'s mirror
image of France — Sicily and Sardinia push its largest-part centroid 81 km north
of the point that is actually in the data.

Measured: **492 filing centroids** among 56,694 distinct published coordinates —
418 by clause (S), 105 by clause (A), 31 by both — carrying 2,756 records. They
touch **248 of 1,581 point-sourced entries (15.7%)**, and **78 entries have no
unflagged coordinate at all**: every point under them is a filing centroid, so
they are located to an administrative unit and no further. None of those 78
bands above `sparse`.

The flag is evidence, not proof, and **the band is not conditioned on it** — the
band counts distinct coordinates, full stop, and `unflagged_coordinates` is
published beside it so a reader can take the discount themselves.

What the flag cannot see: neither clause is a distance-to-the-real-mine test,
because no such test exists without a gazetteer this pipeline does not have and
would not be transcribing if it did. `AU-NT` is the worked example of both
halves — its `Gove Mine` record filed 400 km inland at `-19.41107,133.36423` is
caught, because that point is the Northern Territory's centroid to 0.62 km and
carries nine differently named mines; its `Bauxite - Australia` record at
`-25.99640,134.99894` is not, because it is a lone rollup on a point that is
neither shared nor within 2 km of any centroid, and only its name gives it away.
Nothing here can tell you that `MA-12`'s `Bou Craa Mine` is 700 km from Bou
Craa either. Clause (A) also carries a measured exposure to coincidence: 5,656
centroids times a 12.6 km² disc is 0.05% of the world's land, so of 56,694
distinct coordinates roughly **27 admin-centre matches are expected by chance**.
That estimate ships in `meta.confidence_ruling_4`, because a flag whose
false-positive rate is unstated is a flag that cannot be argued with.

### The honest headline

**818 of 1,770 point-sourced district-commodity entries — 46.2% — rest on a
single distinct site.** Across all 3,524 entries including petroleum and the
coal fields, that is 23.2%. And 75 of those 818 lone sites are themselves a
filing centroid, so the entry is located to a country or a province and to
nothing finer.

## Unlocated producers

A nation that produced a commodity in 1990 and has nothing located is
**UNLOCATED, not absent** — `unlocated_producers`, keyed by commodity, carrying
the national figure and the reason. Eleven nations for oil, headed by the USA at
7,355 kb/d and Norway at 1,630 kb/d; thirteen for gas; **fourteen for coal**,
headed by France at 12,820 kt. The hole is named and sized so a renderer can
draw it as a hole; it is never filled with a guess.

Coal's fourteen are almost entirely the mines that closed between 1990 and the
sources' 2001-2012 vintage — France's last pit went in 2004, Belgium's in 1992,
Austria's in 2005 — which is stated in the reason on every row and in
`meta.known_gaps.coal_vintage`. Before the coal pass this list would have had
**fifty-nine** nations on it and every one of them would have been an artefact
of reading MRDS as the only coal source there is.

Petroleum rows say **which kind of hole** it is, because the three are not the
same claim:

| `basis` | means | who |
|---|---|---|
| `boundary_sliver_only` | the only overlaps were cross-border digitising ribbons | **USA** |
| `offshore_only` | every province touching it is 99%+ outside every district | **Norway** |
| `no_province_assessed` | no WEP province polygon reaches the nation at all | the other nine |

The last is a gap in the source, not a statement about the ground: WEP 2000
assessed 142 priority provinces worldwide and none of them covers Bahrain's
Awali, Albania's Patos-Marinza, or New Zealand's Taranaki, all of which are
onshore fields that produced in 1990.

The test differs by evidence type, because pretending it does not would be the
same error in a new place. A point source locates a deposit at a coordinate, so
one MRDS record is thin evidence but it *is* a location and any district entry
counts. A province polygon locates a basin, not a field, so a nation whose only
overlap is a coastal clip of a wholly offshore province has nothing located.

## The browser layer

`spheres-web/ui/index.html` gains a **Resources** map shading. It paints one
commodity at a time, hue by commodity and opacity by confidence band, with a
white hatch over the single-site districts, a **brass cross-hatch over the
filing-centroid districts**, and a dashed outline over the unlocated producers'
whole territory. Hovering a district gives its evidence — how many distinct
sites and out of how many records, or which province and how much of the
district it covers, or — for coal — which named coal FIELD covers how much of
it, with the words "coal-bearing GROUND, not output" on the line so the area is
not read as a tonnage. For a petroleum district the hover shows both numbers and
labels them apart: the province total marked SHARED, and this district's
apportioned volume marked DERIVED. Shown together on purpose — a reader who
sees only the second will think a source published it.

The filing-centroid mark leans the opposite way from the single-band hatch, so
where a patch is both weakest *and* centroid-located the two cross, and that
crossing is the most-caveated mark on the map — Centre-Val-de-Loire's bauxite
is exactly it. The hover names the unit: *"single — 1 distinct site from 6
source records: Blanquette/Combercave Mine, La Rouquette/Montplaisir Mine,
Mazaugues Mine · EVERY point here is a FILING CENTROID — 27 differently named
mines on one point: located to an administrative unit and no finer"*. Where
clause (A) caught it the sentence reads *"the centroid of Queensland, 0.4 km
off"* instead, because a named unit is an argument and "a filing centroid" is
only a label. The nation dossier's resource row carries the same count.

The artifact is served whole at `/resources.json` (a route in
`spheres-web/src/main.rs`, `include_str!` of the committed file) and fetched
lazily the first time the shading is opened. Whole rather than reduced to a
render payload on purpose: the provenance, the bands, the admission rules and
the unlocated producers are the point of the file, and a map that cannot show
what is behind a patch is the map this data was cleaned to avoid.

**Opacity there is how many DISTINCT SITES say so, never how much is there.**
The legend says it in those words, under every commodity.

### Two readings, one layer

The layer has a dedicated map mode *and* an overlay, because they answer
different questions and neither substitutes for the other.

* **Resources mode.** The powers recede to scenery grey and the wash is a solid
  fill. The resource layer is the subject.
* **Resources overlay** (`O`, or the last chip in the shading bar). The wash
  becomes a fine diagonal **stipple** in the commodity's hue, laid over
  whatever shading is selected — political, fronts, stability, anything. About
  a third of each pattern cell is inked, so the nation colour underneath still
  reads. This is the reading the mode cannot give: the oil visible *while* you
  are reading the Iraq–Kuwait war, rather than instead of it.

Both are the same eight SVG paths — four bands, the single-site hatch, two
filing-centroid hatches, the unlocated outline — built by the same function;
`overlay` picks the presentation, never the data. The confidence grammar is
identical in both, so it is learned once.

### The filter

A chip per commodity the artifact actually places, plus **all** — a single
neutral steel wash over every district holding anything, banded on the
strongest band among its commodities. `X` / `Shift-X` step it; a commodity row
in a nation dossier isolates that commodity across the world.

The all-view is deliberately **not** coloured by a district's dominant
commodity. That map was built, rendered and falsified: off these sources it
paints Australia rare-earth violet, because MRDS record density is survey
effort, not geology. Do not rebuild it.

Commodities the artifact places nowhere — now wheat and rice — are not filter
options, because there is nothing to filter to. They are named in the legend
instead, with their national figures, so a player who cannot find one learns
that the blank belongs to the sources and not to the world. The list is read off
the artifact and not hard-coded, which is why coal left it on its own the day it
was located rather than needing the UI edited to notice.

### The marks, and the element budget

Above zoom 2.2× each district in view gets **one chip**: a dark plate carrying
up to three commodity glyphs (then `+n`), bordered in the commodity hue at the
confidence band's own opacity. Hand-drawn inline SVG, no icon font, no CDN.

One chip per **district** — never one per record and never one per named site.
A district holding 92 copper records draws the same single mark as one holding
1; the difference is in the border, not in a count of marks. Drawing a mark per
citation would be `meta.do_not`'s survey-effort map in another medium.

The mark layer spends at most **1,200 SVG elements**, counted honestly — the
group, its `<title>`, the plate, every shape in every glyph (uranium and
rare-earths are 5 each, gold 4, oil and iron 2), and the overflow label. So one
oil chip costs 5 and a four-commodity chip costs 15; the budget is spent in
those units, not in "marks". Districts are taken strongest
band first, so an overflow drops the weakest claim rather than an arbitrary
one, and **the number it could not draw is printed in the legend**. Nothing is
hidden by that cap: the wash underneath is complete at every zoom, which is
also why there are no marks at all below 2.2× — a culled sample of 1,471
districts would be a claim about which places matter, and the wash never culls.

Measured on this artifact: **at most 8 wash paths** at any zoom for any filter
— four bands, the single-site hatch, the two filing-centroid hatches, the
unlocated outline — and fewer where a band is empty (rare earths has no
`strong` district, no filing centroid and no unlocated producer, so it draws
4). The whole map peaks near 2,500 SVG elements with the marks live, against
~1,300 without.

### The dossier

Every nation's card carries **What is under its ground**: its districts per
commodity with the band split and how many of them stand on a filing centroid,
the transcribed 1990 national figure beside them marked *NATIONAL, not
divided*, then two separate absences — commodities it
produced that are located elsewhere but not here, and commodities located
nowhere on earth for anyone. The USA's card is the file's largest hole, stated:
7,355 kb/d of oil and 17,810 bcf of gas, unlocated.

### What the layer cannot say

A collapsed disclosure under the legend prints `meta.do_not`, `meta.doctrine`,
`meta.confidence`, `meta.apportionment` and the relevant `meta.known_gaps`
**verbatim**, beside a live count of districts per nation for the current
filter — the survey-effort skew measured off the loaded file rather than
asserted. Its totals agree with the generator's own `districts_with_presence`
(oil 798, copper 411, bauxite 121, coal 328, any 1,471), which is a cheap
standing check that the UI is counting what the generator counted.

Norway is the case that forced the layer to explain itself. It is dashed
UNLOCATED for oil *and* has one washed district, Nordland, because
Vestfjord-Helgeland grazes 0.2% of it. Both the legend and the dossier row say
so in as many words. Deleting the patch to tidy the picture would be the same
fabrication as inventing one.

### Degradation

Every reader guards. A server predating the `/resources.json` route renders the
mode and the overlay with no fills and the reason written where the legend
would be. An artifact without `confidence`, without `provinces`, with a
commodity this UI has no glyph or colour for, or with nothing placed at all,
renders without throwing — and a filter naming a commodity the loaded artifact
does not place says *"this is an empty filter, not an empty world"* rather than
showing a blank map.

## Inputs (staged under `spheres-web/data/`, untracked)

All are public domain (U.S. Government works). Sizes and SHA-256 digests are
pinned in the artifact's `sources` block, so a changed source is detectable.

| file (untracked) | bytes | used for |
|---|---|---|
| `mrds-csv.zip` | 25,791,223 | USGS MRDS 20160315 — deposit LOCATIONS: iron, copper, bauxite, gold, uranium, phosphate |
| `pp1802_shp.zip` | 168,795 | USGS PP1802 — deposit LOCATIONS: cobalt, REE, platinum-group |
| `wep_prva.zip` | 1,511,529 | USGS World Petroleum Assessment 2000 — oil/gas province OUTLINES and volumes |
| `INTL.zip` | 24,126,861 | EIA International Energy Statistics — 1990 national oil, gas, coal |
| `ds896-copper.xlsx` | 87,835 | USGS DS896 — 1990 national copper mine production |
| `ds896-iron-steel.xlsx` | 87,823 | USGS DS896 — 1990 national iron ore, gross weight |
| `ds896-aluminum.xlsx` | 67,751 | USGS DS896 — 1990 national bauxite |
| `psd_grains.zip` | 2,870,854 | USDA FAS PSD — 1990 national wheat and milled rice |
| `minfac-csv.zip` | 297,647 | USGS Mineral Operations Outside the United States — coal MINE locations, world ex-US |
| `china-coal-mines.zip` | 171,830 | USGS OFR 2014-1219 — 2,440 named Chinese coal mines |
| `fsucoal_deposit.shp` | 242,668 | USGS OFR 01-104 — 163 named FSU coal deposit and basin OUTLINES (geometry) |
| `fsucoal_deposit.dbf` | 17,830 | the same layer's names and ages (both parts digested; a changed `.dbf` would rename every basin) |
| `us-coalfields-gis.zip` | 230,322,811 | USGS OFR 2012-1205 — coal field OUTLINES of the conterminous US. Staged as published: the archive is the whole GIS project and the layer read is one 4.8 MB shapefile inside it |

Two committed inputs are read as well: `spheres-sim/data/districts.json` (the
authoritative roster) and, via `tools/terrain`, `ne_10m_admin_1.geojson` and
`spheres-web/ui/index.html` for district geometry and identity.

### Sources deliberately rejected

Recorded in the artifact's `sources_rejected` block so each absence is a
decision on the record rather than an oversight.

- **FAOSTAT** — reachable and rich, but FAO bolts a commercial restriction onto
  its CC BY 4.0 terms. USDA PSD covers wheat and rice for 1990 and is public
  domain, so nothing here needs FAO.
- **USGS Minerals Yearbook PDFs** — the only route to 1990 national production
  for gold, uranium, phosphate and REE, but `pdftotext -layout` misaligns the
  columns: the 1994 gold chapter renders Australia's 244 t against Belize.
  Extracting it would have made Belize the world's second gold producer. Those
  four commodities ship with locations and **no magnitude** instead.
- **USGS DDS-60** — ~600 MB of regional report bundles for field geometry that
  `WEP_PRVA` supplies in 1.5 MB.
- **Global Energy Monitor's Global Coal Mine Tracker**, **the USGS World Coal
  Quality Inventory** and **MSHA** — probed by the coal pass. Each was reachable
  and each was rejected for a stated reason; see *The 2026-08-31 coal pass*
  above, and `sources_rejected` in the artifact.

## Output

`spheres-web/data/district_resources.json` (committed, ~2.3 MB), keyed by
district id so a resource follows its district through the 1991 dissolutions —
375 ids carry both a predecessor and a successor owner, and nothing is keyed by
nation.

```
meta                 doctrine, the do-not-rank warning, what the apportionment is and
                     is not, the confidence bands, the MRDS vocabularies with a decision
                     per value, the 2026-08-31 correction record, the dropped boundary
                     ribbons with their measurements, placement stats, gaps
sources              per-source title, url, licence, role, caveat, filter, bytes, sha256
sources_rejected     what was examined and why it was not used
commodities          per commodity: location source, magnitude source, coverage counts,
                     the source token and what STAGE that token names. Coal names all
                     FOUR of its location sources and carries a vintage warning
national             1990 production by nation — NEVER divided among districts
provinces            every WEP province: volume, how much lies inside any district,
                     whether it is offshore, and the zero-volume ones by name
unlocated_producers  produced it in 1990, nothing located — marked, never faked
unplaced_provinces   petroleum provinces overlapping no district, volumes preserved
districts            { district_id: { commodity: evidence } }
```

A point-sourced commodity records the exact record count, the development-status
breakdown, its confidence, and up to 8 named sites chosen by sorted record id.
`records` and `distinct_coordinates` are different numbers and both ship; the
band reads the second. Each sampled site carries the coordinate it was filed at
and whether that coordinate is a filing centroid, so the flag can be checked one
record at a time:

```json
"MA-12": { "phosphate": { "src": "mrds", "n": 5,
                          "bands": { "Producer": 5 },
                          "confidence": {
                            "basis": "mrds_records", "banded_on": "distinct_coordinates",
                            "records": 5, "distinct_coordinates": 2,
                            "unflagged_coordinates": 0, "centroid_coordinates": 2,
                            "centroid_records": 5, "centroid_stacked": true,
                            "band": "sparse", "superseded_band_on_records": "moderate",
                            "centroids": [ { "at": "31.88318,-6.31617", "records_here": 3,
                                             "distinct_names": 4, "records": 4,
                                             "why": [ "stacked_names" ] } ] },
                          "sites": [ { "id": "10021376", "name": "Bou Craa Mine",
                                       "band": "Producer", "at": "31.88318,-6.31617",
                                       "at_centroid": true } ] } }
```

A province-sourced commodity records the transcribed province total and the
derived apportionment side by side, each labelled:

```json
"IQ-BA": { "oil": { "src": "wep_prva", "province_level": true,
                    "confidence": { "basis": "province_overlap", "provinces": 2,
                                    "max_area_frac_district": 0.898,
                                    "onshore_provinces": 2, "band": "strong" },
                    "apportionment": { "basis": "area_frac_province", "derived": true,
                                       "total": 16139.4, "units": "MMBO" },
                    "provinces": [ { "code": "2024", "name": "Mesopotamian Foredeep Basin",
                                     "known": 292442.0, "units": "MMBO", "o_g": "Oil",
                                     "shared": true,
                                     "apportioned": 16032.5,
                                     "apportioned_is_derived": true,
                                     "intersection_sqkm": 15360.0,
                                     "area_frac_district": 0.898,
                                     "area_frac_province": 0.054823,
                                     "offshore_or_outside_roster": false } ] } }
```

A coal entry is the one place both shapes can occur together. It carries
`sites` where a survey named mines and `fields` where one drew an outline, and
where it has both it states both bands and takes the stronger:

```json
"UA-14": { "coal": { "src": "fsucoal+minfac", "n": 1,
                     "bands": { "Active": 1 },
                     "sites": [ { "id": "...", "name": "Donets Basin",
                                  "src": "minfac", "year": "2007",
                                  "commodity": "Coal: hard",
                                  "where": "Donets'ka", "at": "48.00000,37.68000",
                                  "at_centroid": false } ],
                     "field_level": true,
                     "fields": [ { "src": "fsucoal", "name": "Donetsky basin",
                                   "polygons": 1, "dep_age": [ "C" ],
                                   "sqkm_field_total": 60301.4,
                                   "intersection_sqkm": 10382.35,
                                   "area_frac_district": 0.3907,
                                   "area_frac_field": 0.172175,
                                   "note": "a coal-bearing AREA the survey named ...
                                            No tonnage exists in this source and
                                            none is derived from it." } ],
                     "confidence": { "basis": "coal_mine_points+coal_field_overlap",
                                     "banded_on": "stronger_of_points_and_fields",
                                     "records": 1, "distinct_coordinates": 1,
                                     "band_on_points": "single",
                                     "coal_fields": 4, "max_area_frac_district": 0.3907,
                                     "band_on_fields": "moderate",
                                     "band": "moderate" } } }
```

**Districts with nothing sourced do not appear.** 1,471 of 2,610 do — 56.4%.
3,524 district-commodity entries, of which 328 are coal.

## How placement works

Deposits attach in **WGS84 lon/lat**, not projected canvas space. `geo.py`
imports `derive_districts`, `project` and `shoelace` wholesale from
`tools/terrain/classify_districts.py` rather than reimplementing them — that
function replicates `mapgen.rs`'s identity rules exactly (the `AGGREGATE` set,
the ISO-3166-2 uniqueness test, the slug fallback, the `-2` suffixing), so a
resource and a terrain class on the same id refer to the same ground by
construction. There is one definition of "district AF-BAL" in this repo and it
lives over there.

Matching uses the source geometry rather than `spheres-web/ui/districts.js`
because the latter is rounded to 0.1 px (~1.5 km) and simplified; a coastal mine
can fall outside its own district's drawn outline. The projection is a display
transform, identity is geographic. `geo.py`'s self-test pins nine landmarks —
Kabwe, Chuquicamata, Kalgoorlie, the Witwatersrand, Ghawar, the Ruhr, Kolwezi,
Krivoy Rog, Houston — and all nine resolve to the correct district:

```
python tools/resources/geo.py
```

Points outside every district may snap to the nearest centroid within 0.75°;
beyond that they are dropped. Both counts are published per commodity in
`meta.placement`, and together they are under 1% of placed records.

## The nation crosswalk

`crosswalk.py` is hand-authored, one table per source vocabulary, and
`geo.NationCrosswalk` **raises on an unmapped name that carries data**. A silent
drop reads as "this country has no resources", which is a fabrication — the
probe's naive matcher lost 49 nations including the USA and the USSR that way.

Only the national tables need it. Point sources are placed by coordinate and
never by name, so MRDS's modern 2016 vocabulary never has to be reconciled with
the game's 1990 roster.

The 1990 vintage mostly selects itself: EIA reports `--` for Russia, Ukraine and
Kazakhstan in 1990 and puts the production under `Former U.S.S.R.`, and DS896
writes `XX` for republics that did not exist. Two merges are editorial and are
documented in `crosswalk.py` rather than hidden: `Germany, East` + `Germany,
West` → `Germany` (the roster has one Germany; reunification was Oct 1990), and
`Yemen (Aden)` + `Yemen (Sanaa)` → `Yemen` (unified May 1990). Both keep their
source labels in the artifact and are flagged `merged: true`. USDA PSD is the
exception that does **not** use 1990-era polities — it back-casts the USSR into
successor republics — and those rows are kept as reported and flagged rather
than re-aggregated.

## Known gaps — read before building on this

Also carried in the artifact's `meta.known_gaps`.

- **No US oil or gas.** The World Petroleum Assessment 2000 assessed provinces
  *outside* the United States; every North American polygon stops at the 49th
  parallel. The USA was the world's #2 producer of both in 1990. National
  figures are present, locations are not. This is the largest hole. Exact
  clipping *does* find three US overlaps and all three are digitising ribbons
  along a shared border; they are dropped and named at
  `meta.boundary_slivers`.
- **Offshore petroleum is where Norway's and Britain's oil is.** Five provinces
  overlap no district at all and keep their volumes in `unplaced_provinces`.
  Others clip a coastline — the North Sea Graben reaches 8.4% of one
  north-eastern British district, Vestfjord-Helgeland 0.2% of Nordland, though
  99.7% and 99.9% of those provinces lie outside every district. Those
  attachments are measured and kept, flagged, and banded `single`; both nations
  are listed as unlocated oil producers.
- **Wheat and rice are national-only.** No dataset places a wheat field.
  National-only by refusal, not omission.
- **Coal is located, but not at 1990.** 328 districts in 63 nations carry it and
  45 of the 59 nations that mined coal in 1990 have it located — but all four
  location sources date from 2001-2012 and none is a 1990 census. The hole that
  remains is VINTAGE, not absence, and it understates 1990: the Ruhr shows nine
  collieries and not twenty, British coal shows the twenty pits left in 2007,
  and France, Belgium, Austria, Italy and Portugal are unlocated because their
  last pits closed between 1992 and 2005. See *The 2026-08-31 coal pass*.
- **A coal field is not a mine, and 161 coal districts rest only on one.**
  fsucoal and uscoalfields map coal-bearing GROUND; OFR 2012-1205 says outright
  that it does not distinguish minable from uneconomic coal. Those entries
  publish an AREA fraction, band on it separately from any mine evidence, and
  say so on the hover line.
- **`n` counts minfac ROWS, and one row can be seventeen mines.** "Mine at Upper
  Silesia (17 mines)" is one row; eight such rows stack on 50.17N 18.83E, which
  is why `PL-SL` reads `single` and carries the filing-centroid flag. The
  Yearbook reports at whatever resolution its correspondent had, and China is
  eleven province rollups in minfac — which is why china2014 is carried beside
  it rather than instead of it.
- **Guinea — the world's #2 bauxite producer — is absent, and the artifact does
  not say so.** Found by `check.py`'s ground-truth pass on 2026-08-31; NOT yet
  fixed, and named here rather than quietly carried. DS896 records Guinea at
  **15,800,000 t** of 1990 bauxite: 14.0% of the 113,000,000 t world total,
  rank 2 behind Australia and half again Jamaica's 10.9 Mt. MRDS locates ten of
  its mines correctly and by name — Boké-Sangarédi, Sangarédi, Kindia, Kindia
  Débélé, Kindia-Friguiagbé, Friguia, Fria, CBG Boké, and two `Bauxite -
  Guinea` grid points — at real Guinean coordinates (10068650 sits at
  11.1675N 13.74859W, which is Sangarédi).

  All eleven data points are dropped, and **not because of any defect in the
  sources**: the district roster in `spheres-sim/data/districts.json` models 160
  nations and Guinea is not one of them, so there is no polygon for the mines to
  fall inside, and `crosswalk.IGNORE` lists `"Guinea"` among the "sovereign
  states outside the 160-nation roster" so the production figure is skipped
  before `national` is built. The IGNORE decision is defensible on its own
  terms. **The silence is not.** The doctrine is absence *plus an explicit
  unlocated marker*, and there is no marker: Guinea appears in neither
  `national.bauxite` (24 nations) nor `unlocated_producers.bauxite` (Romania and
  Albania only), and the string never reaches the shipped file.

  The fix is not in this directory. Either the roster gains Guinea — which is
  the other swarm's territory — or `make_resources.py` learns to emit an
  `unrostered_producers` block so the crosswalk's own drop list becomes visible
  in the artifact instead of living only in the generator source. Until one of
  those happens, `check.py` asserts every component of the hole (roster has no
  Guinea; MRDS holds exactly 10 admitted records; none is cited; the name is in
  neither national block) so that it cannot change size without the checker
  noticing, and WARNs with the numbers on every run.

- **`n` still counts records, not sites — but nothing bands on it any more.**
  Ruling 4 moved the band to `confidence.distinct_coordinates` and `n` is now
  what it always was: a citation count, kept because it is the transcribed
  truth about how many records the source holds. The stacking it measures has
  not gone away and cannot: 5,699 of the 63,119 admitted point records share a
  coordinate with another, across 1,887 coordinates, and 46.56346N 2.55405E —
  the centre of France — carries 27 of them. What changed is that
  `FRA_centre-val-de-loire` bauxite now reads `single` rather than `strong`,
  because one point is one site whatever six records call themselves. Related:
  `FRA_auvergne-rhône-alpes`'s lone bauxite record is 10022538 "Sangaredi" — a
  *Guinean* mine whose MRDS row reads `country = France`, `state = "Department
  of Guiana"`, at 46.52973N 2.71711E. The coordinate is transcribed as
  published, because a corrected one would be an invented one.
- **Six commodities have no magnitude** — gold, uranium, phosphate, cobalt,
  rare_earths, platinum_group. DS896 does not cover them and the Yearbook PDFs
  could not be parsed safely.
- **PP1802 has no status field.** cobalt, rare_earths and platinum_group come
  from a deposit catalogue with no `oper_type` and no `dev_stat`. Rules 1–3
  cannot be applied to them and are not: those entries are deposits, which may
  never have been mined. Their confidence basis says `pp1802_deposits` for that
  reason.
- **Some MRDS records carry an administrative centroid, not a site
  coordinate.** Six French bauxite mines of the Var all sit at 46.563N 2.554E —
  the centre of France — which is why a bauxite district still appears in
  Centre-Val-de-Loire; four of Weipa's six records sit at the centroid of
  Queensland; `Bauxite - Australia` sits near the centre of Australia. **The
  coordinate is transcribed as published and is never corrected**, because a
  corrected coordinate would be an invented one. What ships instead is the
  filing-centroid flag above: 531 such coordinates are named — the coal pass
  added 39 by putting 2,798 more point records through the same census, which
  is where "Mine at Upper Silesia" got caught, and re-running the generator with
  coal excluded from the census alone changes **zero** non-coal entries, so
  nothing that shipped before moved because coal arrived — the entries
  touching them carry `centroid_stacked`, and the map hatches them and says so
  on hover. A lone rollup like `Bauxite - Australia` still escapes both clauses
  and is called out by name in `meta.confidence_ruling_4`.
- **Never rank by `n`.** It counts citations, not ore.
- **`apportioned` is derived, `known` is transcribed, and 1990 production is
  neither.** The first is an area weighting of a 2000-vintage resource
  assessment; the second is that assessment's own province total; the third is
  a national figure in the `national` block that is never divided. Three
  different things, and the artifact labels all three.
- **Al-Basrah is not Iraq's arithmetic top oil district and cannot be made
  one.** It out-ranks Al-Anbar decisively — 16,139 MMBO to 12,927, `strong` band
  to `moderate` — which is the defect the rebuild was for. But the Mesopotamian
  Foredeep covers Sala ad-Din, Wasit, Dhi-Qar, Maysan, Babil and Baghdad too,
  and WEP publishes one volume for the whole basin. Rumaila and West Qurna are
  not in this source. Ranking Basra first would mean weighting by field
  knowledge nothing here carries.

## Determinism

No RNG, no wall clock, no set iteration reaching the output. Districts and
commodities emit through sorted keys, deposits sort by source record id, the
STRtree is built over district ids in sorted order and its query results are
sorted before use, and the JSON is written with `sort_keys=True`. GEOS clipping
is a deterministic function of its inputs, so the floats repeat.
`check_resources.py` regenerates the file and requires the SHA-256 to be
unchanged.

Verified 2026-08-31, three consecutive runs after the coal pass:

```
66436b4860621dbf2cb07257fac232ac8adc2f78eedb4102b65d644059090f03   4,577,848 bytes
```

The four coal sources add no new source of non-determinism: the two point
sources emit through sorted keys like MRDS, and the two polygon layers are
walked in `(name, record index)` order and aggregated through a sorted dict, so
the GEOS clipping that produces their areas is fed the same geometries in the
same order on every run.

### One thing the area check caught

The exact area integral is `−∮ sin φ dλ`, and the obvious closed form for an
edge — `−Δλ (cos φ₁ − cos φ₂) / Δφ` — is a trap. GEOS emits edges cut along a
latitude line whose two latitudes differ in the last bit or two, and the
subtraction of two nearly equal cosines then cancels away every significant
digit. On the Pannonian Basin's clip of Tuzla one such edge (Δφ = −6.7e-16,
Δλ = −3.0e-3) was wrong by 5% of a term the size of the whole polygon, and the
district measured 6,129 km² against a true 3,047. The identity
`cos A − cos B = 2 sin((A+B)/2) sin((B−A)/2)` rewrites the same integral as
`sin(midpoint) · sinc(Δφ/2)`, which has no subtraction in it at all. That is the
form in `geo.ring_area_sr`, and the check that holds every measured district
area against `districts.json`'s independently produced `area_sqkm` is what found
the bug — it was invisible in the fractions, which are ratios and cancelled it.
