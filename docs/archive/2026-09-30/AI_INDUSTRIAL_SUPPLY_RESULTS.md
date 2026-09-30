# AI industrial supply manager — verification results

Local-review evidence, 2026-09-04. These results describe one deterministic
six-year run. They verify accounting and behavior for the reviewed seed; they
are not a claim that adoption rates are calibrated across many seeds.

## What was run

- Seed `42`, 2,192 daily ticks, 137 starting countries.
- Corrected report:
  `../../artifacts/materials-supply-corrected-2192-42.json`.
- Census state hash: `e5551983ad19dbea`.
- Report SHA-256:
  `BBB86202D4E8624E4DEF4A516B21CDC6D88855F232600C381D8638AD9C2E90A2`.
- Comparison points are the earlier Materials-AI baseline and the first supply
  manager census made before the two audit fixes.
- The census applies no gifts, forced orders, macro overrides or target adoption
  bar. It records real daily work, deliveries, consumption and project state.
- Six countries remain geographically unmapped in the staged world data; all
  mapped countries have a Materials allocation and all 137 remain in the audit.

Every invariant passed: finite daily balances; quarterly GDP and Materials
reconciliation; frozen inherited capacity; no initial gifts; exact save
roundtrip; and byte-for-byte state plus RNG equality over a 30-day resumed
continuation.

## Six-year result

| Measure | Materials-AI baseline | First supply build | Corrected supply manager |
|---|---:|---:|---:|
| Countries ordering / producing inherited Materials | 31 / 31 | 26 / 26 | **28 / 28** |
| Orders started / completed / expired / active | 73 / 69 / 2 / 2 | 153 / 121 / 28 / 4 | **279 / 254 / 14 / 11** |
| Blocked order-days | 4 | 214 | **23** |
| Inherited Materials delivered | 537.809 | 2,110.124 | **2,324.050** |
| Countries receiving / delivered imported intermediates | 22 / 2,896.326 | 31 / 3,510.425 | **30 / 1,428.500** |
| Countries with paid processing / packs produced | 86 / 13,765.149 | 71 / 12,214.528 | **72 / 12,677.246** |
| Countries starting / installing / producing machinery | 34 / 34 / 34 | 28 / 27 / 27 | **33 / 30 / 30** |
| Domestic capital-goods packs | 4,685.524 | 3,971.810 | **3,742.328** |
| Countries using goods in prototype research | 0 | 2 | **2** |

The manager is changing routes rather than creating supply. It uses less than
half the manufactured imports of the first build while delivering more finite
domestic Materials. Capital output is lower than the older baseline because
several machinery projects start later; a project start is intentionally not
counted as installed capacity or production.

Of the contracts that reached a closed state, 94.776% completed, up from
81.208% in the first supply build. Eleven orders were still legitimately active
at the census cutoff.

## Result by opening economic size

| Opening GDP | Countries ordering / producing | Orders started / completed / expired | Blocked days | Materials delivered | Importers / imports | Machinery starts / installed / producing |
|---|---:|---:|---:|---:|---:|---:|
| Under $1bn | 0 / 0 | 0 / 0 / 0 | 0 | 0 | 0 / 0 | 0 / 0 / 0 |
| $1–10bn | 0 / 0 | 0 / 0 / 0 | 0 | 0 | 0 / 0 | 0 / 0 / 0 |
| $10–100bn | 13 / 13 | 112 / 101 / 6 | 10 | 898.960 | 12 / 794.500 | 13 / 13 / 13 |
| $100–1,000bn | 12 / 12 | 133 / 122 / 7 | 10 | 1,205.216 | 13 / 496.500 | 15 / 14 / 14 |
| Over $1,000bn | 3 / 3 | 34 / 31 / 1 | 3 | 219.874 | 5 / 137.500 | 5 / 3 / 3 |

Micro and small economies retain their proportional intermediate-goods modules,
but the manager does not force them into a full-size machinery chain. Their zero
rows above are therefore a deliberate affordability and demand boundary, not
missing census coverage.

## The two audit fixes

### Starter imports no longer churn back onto the market

In the first build, France, Japan and Germany imported 359.041, 299.324 and
163.696 intermediate packs while never starting machinery. Their partial
starter lots were repeatedly exposed for resale. In the corrected run each
imports exactly 15 packs, is never named as the seller of those packs, and starts
machinery on days 1,581, 1,246 and 1,916 respectively. France and Japan install
within the census; Germany's later project remains honestly unfinished at the
cutoff.

The fix is a bounded, evidence-latched 15-pack export reserve. It starts only
after real stock, paid inbound freight, a finite domestic contract or recent
output demonstrates accumulation. Everything above the starter lot remains
tradable.

### A one-day raw quote is no longer a 30-day promise

The first build incurred 202 bauxite-blocked order-days because today's possible
flow was multiplied into a month-long contract. The corrected manager caps a
finite order to the complete unreserved raw bundle already owned. Bauxite blocks
fall from 202 to **zero** and total blocked order-days fall from 214 to 23.

The remaining 23 blocked days are temporary operating-authority interruptions,
not missing raw stock. Fourteen finite orders expire after such interruptions;
this is visible loss rather than silent or free completion. Future balancing can
decide whether contract windows should include slack, but the accounting and
physical gate are behaving as specified.

## Test and interface evidence

- Supply-manager regressions: 16 passed.
- Economic Competition regressions: 18 passed.
- Materials AI bootstrap regressions: 12 passed.
- Full non-calibration Rust workspace: 735 passed, 57 ignored, 3 known
  calibration/golden checks filtered, 0 failed.
- Current web suite after campaign-binding Exchange reads: 135 passed, 2
  ignored, 0 failed.
- Current non-browser UI checks: 184 passed, 0 failed (319 total checks when the
  web Rust suite is included).

The review UI was also exercised against the running server. Economic
Competition remains an explicit opt-in. The player sees live 90-day Materials
and Machinery cards; the World view exposes each government's dated post-action
snapshot, including Need, Covered, Gap, funding outlook and plain-language
reason. Exchange snapshots and quotes require the exact active campaign session,
so an older browser tab fails closed instead of rendering another campaign's
ledger.

## Honest next verification

Before tuning policy constants, run a multi-seed census and compare the
distribution of imports, machinery timing, authority-related expiries and
research use. The current run establishes a safe deterministic foundation; it
does not by itself choose an ideal rate of industrialization.
