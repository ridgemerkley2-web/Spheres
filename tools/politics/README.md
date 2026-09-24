# Opening military executive-removal leverage

The catalog supplies a separate initial input for military influence. General
authoritarianism also represents elections and civilian institutions; it cannot
by itself establish the military's position relative to an executive.

The inputs are from **Coppedge et al. (2026), V-Dem Country-Year Dataset v16**,
[dataset DOI](https://doi.org/10.23696/vdemds26), distributed by the
[V-Dem Institute's official R package](https://github.com/vdeminstitute/vdemdata).
The importer pins commit `f4dd26922e658442524dfd954bf14f7ebe622d5d` and verifies
the original RData SHA-256 before extracting the 158 source rows from **1989**.
The checked-in subset has its own fixed SHA-256. Full source receipts remain in
the campaign verification evidence. No 1990 or later observation is imported.

The source removal fields (`v2exrmhsol_4`, `v2exrmhgnp_4`) concern the military's
practical ability to remove the head of state or government without using armed
force. Their source means are weighted by relative executive cabinet powers
(`v2ex_hosw`, `v2ex_hogw`). This combination is a **gameplay inference**. It is
not a coup probability, troop loyalty, the strength of a defending force, or a
measurement of physical Army autonomy. See the [v16 codebook](https://www.v-dem.net/documents/70/codebook_v16.pdf).

These are annual, lagged assessments, not observations made on January 1, 1990.
Late-year transitions can make them stale. A known zero differs from an unknown
source; neither guarantees immunity to underfunding, war, or an armed coup.
Historical input and live campaign authority are stored separately. Loading an
older save never invents the missing assessment or replays historical events.

The crosswalk covers 130 of 137 opening countries. Five have no source row;
Germany and Yemen are also left unknown because the game's unified opening
polities do not match the separate 1989 source units. Historical USSR, Czechoslovak
and Yugoslav units follow the [official country-unit guide](https://v-dem.net/documents/72/countryunit_v16.pdf),
with the Yugoslav/Slovenian boundary recorded explicitly. There is no automatic
inheritance by later successor countries. All original components and weights
are retained. Missing positive-weight components and invalid weights fail validation.

## Reproduction

Normal checks require Python's standard library:

```sh
python -B tools/politics/build_army_authority.py --source tools/politics/fixtures/vdem-v16-1989-military-authority.json --nations spheres-sim/data/nations --output spheres-sim/data/army_authority_1989.json --check
python -B -m unittest discover -s tools/politics -p "test_*.py"
```

To independently reproduce the small input from the pinned official RData,
install `pyreadr` and use `extract_army_authority.py RDATA OUTPUT`. The extractor
checks both the downloaded input and reproduced output hashes before writing.

## Data attribution and license

The source subset and derived catalog are adaptations of V-Dem data and are
shared under **[CC BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0/)**,
as required by the [V-Dem data license](https://www.v-dem.net/about/faq/).
Changes consist of a 1989 subset, a documented game-country crosswalk and an
executive-power-weighted proxy. This notice applies to
`fixtures/vdem-v16-1989-military-authority.json` and
`../../spheres-sim/data/army_authority_1989.json`. Program code retains its own
repository license. V-Dem does not endorse the game or this modeled interpretation.
