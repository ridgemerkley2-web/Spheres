# S22 contract route reuse: independent source review

Reviewed production/test commit: `1a4b7698` (base `9823b070`).

Review date: 2026-09-28. Reviewer: collaborating agent `/root/review_gap_submission`.
This note records that agent's actual read-only review message. It is not a human approval, executed test report, timing qualification, or S22 closeout.

Scope: `spheres-sim/src/resources.rs`, `logistics.rs`, `freight_routing.rs`, and `logistics_contract_dispatch_tests.rs`.

The reviewer found no production-blocking invariant issue:

- The nominal-search context exists only during the ordered resource contract-dispatch pass.
- Dispatch mutates cargo, freight usage, cash and company experience; it does not change topology, access, ownership or route policy within that pass.
- Terminal capacity remains uncached during route assembly. Congestion selection and bundle capacity ratios retain live capacity reads and no spot-market search budget.
- Complete route plans are rebuilt for each bundle; existing nominal search-tree ordering and tie behavior are unchanged.
- Focused tests exercise congestion, paid terminal-experience thresholds and rebuilding the context after later-day changes.

The reviewer identified an oracle input-format limitation: the initial ignored test accepted an integrated simulation save, but not the outer campaign envelope. A separate test-only followup adds campaign-envelope extraction by moving its existing world JSON value; integrated capability envelopes remain intact. Remaining campaign history is dropped before world decoding/cloning. The followup includes a regression for both formats and does not change production routing.

Neither reviewer nor implementing agent ran builds/tests for this patch. Root coordinates those runs and actual 2015/2035 checkpoint comparisons. The ignored oracle must exercise multiple nonzero contract shipments and match the original per-leg router's complete world and returned/retained headlines for every one of 31 consecutive native days. It does not add player orders or renew budgets, and is separate from browser/native performance qualification.
