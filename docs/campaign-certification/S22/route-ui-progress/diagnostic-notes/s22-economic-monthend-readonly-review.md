# S22 month-end economic AI review (read-only)

Source: `s22-subsystem-2006-stockfix-isolated-01/profile.json`, revision `a96bb097b78f`, the row beginning 31 Jan 2006. No commands, replay, builds, or benchmark runs were launched in this investigation.

Economic AI measured 243.1917 ms; politics 97.754 ms; military AI 14.147 ms. These component diagnostics identify work, not a qualification claim. Nested country totals were excluded from the leaf aggregation below.

| Leaf stage | Calls | Total ms |
| --- | ---: | ---: |
| candidate | 98 | 66.8282 |
| purchase | 157 | 31.5090 |
| record.raw_supply | 155 | 27.5484 |
| mine | 108 | 25.4023 |
| offer_surplus | 174 | 25.2285 |
| initial raw_context | 1 | 22.1765 |
| record.supply | 155 | 20.3532 |
| commission | 141 | 13.9345 |
| bootstrap | 139 | 8.3266 |

The largest complete country review is Cuba at 7.9362 ms, followed by Tunisia 6.6903, Iran 6.5903, Russia 6.5715. This is distributed repeated work, not one dominant exceptional command. Multiple candidate/purchase/commission entries can belong to one country and reflect separate pure decision and enactment phases; they were not treated as one call.

## Concrete narrow recommendation

In `spheres-sim/src/economic_ai.rs`, `mine_for_shortage` constructs the complete `RawSupplyForecast` (all twelve lines, three horizons and descriptions), reads only RUN shortage while choosing a mine, and discards the report. Both caller branches subsequently invoke `record`, which builds the same complete report again if no command intervenes.

The existing observer sequence proves all 108 measured mine calls were followed by `record.raw_supply` with **zero intervening review.execute attempts**. All 108 therefore selected no mine. Their repeated final raw-report calls took **18.6218 ms** in aggregate; the mine calls themselves took 25.4023 ms. All mine calls lasted at least 0.0849 ms (median 0.2199, max 0.9235), but elapsed time is not proof of which early guard ran.

Proposed bounded change: return/retain an owned completed raw forecast from mine preflight and reuse it only for the immediate no-command record branch on the same world. If an execution is attempted, discard it before that attempt, even if the command refuses. Early exits that never produced a forecast continue through the usual record read. This changes no decision thresholds, <=/NaN semantics, commodity iteration order, raw horizon, consumer arithmetic, opening context, order pricing, or persisted report schema. The read-only forecasting functions remain the single formula owner. Exact whole-world and serialized report parity against the old review path are required before acceptance.

A separate lazy guard could postpone forecast creation until the first commodity with `!(demand <= stock)`; do not rewrite that as `demand > stock` without a proven finite-input contract. The existing artifact does **not** retain per-call automatic demand/stock values or the evolved Jan31 world, so the exact count that would benefit from this lazy guard cannot be recovered from timers. A Jan1 source proxy would not establish a Jan31 count. No source parse/replay was started alongside the parent's quiet performance tests.

No implementation is included. Resource and logistics files remain with their assigned owner.
