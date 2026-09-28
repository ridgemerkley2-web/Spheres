# S22 pair02 candidate source review

Runtime review: `070141be78be39db0a66db09026c2887831acb8b`.
Annual-plan fixture correction: `a7356a98413c020f4629df484a4d6ce9e4de55ca`.
Final validation candidate: `5d11dd6dae436eeca105dbaf0d02732cd768b35b`.
These are source reviews, not executed correctness or qualification results.

The root reviewer and an independent reviewer inspected all four optimizations:

- `fe15010b`: deployment edge capacities are keyed by exact sea-factor bits within an immutable world/graph borrow. The cache is dropped before mutable supply settlement. Heap/search/filter/arithmetic order is retained.
- `09c9c8cc`: factory energy calculations cache only fixed generation geometry and inherited headroom within one operating pass. Company modifiers remain live; industry and GDP retain their distinct original arithmetic expressions. Later material operations receive no retained scope.
- `2fb92273`: freight arrival keys borrow existing route strings, with the original ID-only versus ID/name comparison distinction. Only disjoint due-date and hold-reason fields change before the borrows are dropped. Partition order and stable cargo-ID sorting remain unchanged.
- `070141be`: monthly purchasing reuses only ordered seller/buyer route-availability booleans within a single wave. Purchase proposals have Money/Commodity legs and cannot transfer districts. Relations, standing, contracts, offers, refusals and headlines may change, but route planning does not read those values. Supply, cap, prices and decision evaluation remain live.

A second independent reviewer additionally checked the monthly proposal/signing path and found no missed mutable route inputs. No actionable source findings were identified. Original-path switches and actual checkpoint comparisons are retained to test these conclusions at execution time.

Initial execution found an incomplete synthetic fixture in the industry full-day test: the isolated factory helper installed department budgets but no annual plan, so a later complete native day correctly rejected that invalid state. The test-only correction enacts the unchanged allocations through ordinary `SetProgramBudget`, then opens the department day. It retains every parity/reuse assertion and changes no runtime implementation. The initial failure and subsequent independent validation logs are both preserved.

Actual 31-day comparisons then reached exact world/headline equality but exposed unsupported terminal coverage assumptions: the 2015 deployment path had no repeated capacity reads and neither purchase input repeated its route checks. These attempts remain failed tests in the archive. The actual 2035 deployment test passed with 1,120,154 reused edge reads. An independent source/input review found one-material missile recipes in all nine historical purchasing cover buyers, while the positive synthetic test deliberately uses a two-material armour recipe. The counter scope and original-path flag were correct.

The final test-only correction retains positive synthetic coverage and positive actual-2035 deployment coverage, while the earlier save reports the work actually encountered. Both purchasing actual oracles now require exact route-search accounting: optimized calls cannot exceed original calls and every reused result replaces exactly one original search. Complete world/headline comparisons and unchanged source bytes remain mandatory. Zero reuse claims no actual-checkpoint speedup. No qualification limit or measured workload changes.

The failed pair01 remains unchanged: 17/18 cells passed, with late-2035 confirmation simulation/history p95 323.6552 ms and whole-turn p95 406.1905 ms exceeding their unchanged limits. A new candidate must pass a complete new pair; source review and preflights do not qualify S22.
