# 1990 historical-output source collector

This tool produces `spheres-sim/data/industry_1990.json` for all 137 canonical
starting countries. Its quantities size **GAME CAPACITY ESTIMATES**, not literal
historical factories. It never changes nation GDP, physical assets, inventories,
budgets, or saves.

## Rebuild

Node 18+; no dependencies. Run from the repository root:

```text
node --test tools/industry/check_industry_1990.cjs
node tools/industry/collect_industry_1990.cjs --fetch --share-only
node tools/industry/collect_industry_1990.cjs --fetch
node tools/industry/collect_industry_1990.cjs --offline
```

The first fetch obtains National Accounts shares; the second adds INDSTAT mixes
while reusing those cached requests. Offline rebuilding requires the raw cache.
The default cache is `../../artifacts/industry-1990-cache` relative to this
repository, outside the source tree. `--cache-dir PATH` selects another cache;
`--output PATH` permits a comparison artifact without overwriting the game data.
`--max-countries N` or `--only USA,Japan` bounds a collection pass. Every output
still contains 137 rows; unqueried rows remain explicitly labeled fallbacks.

Requests have a 15-second timeout and at most four workers. No browser, bypass,
credentials, hidden service, or retry loop is used. Cached envelopes preserve
request bodies, raw responses, retrieval dates, and SHA256 checksums. Failed
requests are cached too: inspect the error and use `--refresh` for an intentional
retry. `--refresh` refreshes selected metadata and country requests, not just
failures, so it may adopt a newer statistical vintage. Runtime dataset IDs are
resolved from the documented metadata endpoint, never hardcoded.

## Meaning and limits

- National manufacturing share is 1990 `MvaCud / GdpCud`: both National Accounts
  series use current USD in the same year. Source amounts are retained, but the
  ratio is applied to the game's existing GDP by a separate simulation layer.
  Valid reported zero is not missing. Unknown/suppressed values stay null.
- The fallback manufacturing share is the approved existing 20% model preset.
  USSR, Yugoslavia, and Czechoslovakia use explicit fallback, not a sum of modern
  successors. Zaire maps to COD, not COG. Sudan uses the exact metadata reporter
  `Former Sudan`; absent historical coverage is not replaced with modern Sudan.
- INDSTAT Revision 3 monetary value added uses the source's common local `v`
  unit; currency cancels in a within-country mix. The optional `u` monetary
  companion is neither an overlap flag nor proof of the `v` unit being USD.
- Five nonoverlapping **game** groups are fixed: food/textiles 15–19; materials
  20–23 and 25–28; chemicals 24; machinery/electronics 29–35; other 36–37.
  Refining (23) belongs to materials in this model, not chemicals.
- Complete 23-division nonnegative observations normalize to `indstat_1990`.
  Exact same-group `NN includes NN` notes are resolved by counting the valued
  parent once and excluding covered children. Cross-group, nested, conflicting,
  duplicate, or unrecognized aggregates cause a generic profile fallback.
- With at least 18 represented divisions (observed or covered by a resolved
  parent), at most five uncovered missing divisions can receive the median of
  positive, uncombined observed division values. This is the approved
  `partial_1990_model`, **not an observed historical distribution**. Imputed
  amounts and missing codes are listed separately; original nulls stay null.
- Other profiles use five equal 20% weights, labeled `model_fallback`.
  Supplier/scope/publication metadata is retained as provenance, not mistaken
  for row overlap. Coverage counts separately report full, partial-model, and
  fallback mixes.

The committed-ready JSON contains source observations, notes, formulas and
request references. Preserve the cache when exact offline reproduction of its
statistical vintage is required. Re-fetching the public API can legitimately
return revised observations; that is a data update requiring review, not a
byte-identical rebuild. Boundary qualifications and national accounting data do
not prove factory locations, establishment counts, or 1 January capacity.

Official contracts: [UNIDO API documentation](https://stat.unido.org/unido-statistics-portal-api),
[National Accounts metadata](https://stat.unido.org/portal/dataset/getDataset/NATIONAL_ACCOUNTS),
[INDSTAT3 metadata](https://stat.unido.org/portal/dataset/getDataset/INDSTAT/3).
