# Original A1 cohort comparison (checkpoint 25)

This read-only diagnostic replays the unchanged original A1 assertion, seeds 0-11 for 252 months, against checkpoint 25. Its only instrumentation prints complete existing per-country counts. The original assertion remains red: median 7 coups and median top-three share 0.5857142857 (required below 0.5). The test exits 101; no criterion has been replaced.

The comparison also computes the proposed effective-country count, 1 / sum(p_i^2). Median effective count is 49/9 (5.4444444444); all twelve seeds meet the proposed per-seed value of four. Only two seeds meet the old per-seed top-three condition. This is a substantive alternative design criterion, not an equivalent rewrite or an approved passing result. The toy examples document the difference.

The prospective plan and build input retain source and library hashes. The result records successful compilation, the original failed assertion, unchanged before/after library hashes, and executable/log hashes. The external emitter can be reconstructed from the untouched census file; it adds only a non-mutating print. Large binary artifacts remain external. No new seeds or independent holdout were used.
