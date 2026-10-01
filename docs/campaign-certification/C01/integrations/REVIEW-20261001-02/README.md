# Combined C01 review and claim registration — 1 October 2026

Accepted and integrated **Saudi C01-45 and State Duma C01-46 as bounded research**.
The [Saudi review](../../reviews/CLAUDE-C01-45-20261001/README.md) covers 18 originals,
30 claims and 18 new observations. The [Duma review](../../reviews/CLAUDE-C01-46-20261001/README.md)
covers seven originals, 27 claims and 18 new observations. Each received an
independent source/content review and a second focused preservation/correction review.
Their source import, corrective and review commits remain individually reachable.

Tonga C01-44 is **held**: [partial review](../../reviews/CLAUDE-C01-44-20261001/README.md)
read four originals and five claims; the final PMO request returned HTTP 429.
One source/claim remains unread, with no new holders. Only its review receipt was
imported; the isolated country-data import and prose correction remain excluded.
No additional Archive requests or held-source retries followed that response.

Existing C01-42–46 claims are registered without duplicate assignments. Japan
C01-42 remains claimed. Brazil C01-43 at `68fb8f86` is newly submitted and awaits
original-content review; its proposed 14 sources and 21 claims were not imported.
The workboard records **54 bounded tasks, 28 completed Claude tasks and 44 unchanged
canonical markers**. Three submissions are held after partial review; Brazil is
separately unreviewed. Tonga cast production remains the first content priority.

## Combined validation

- **688 avatar tests passed** after repairing one stale exact accepted-packet set.
  The original 688-test run had one failure because its expected set omitted 45/46;
  that output is retained. No source, date, boundary or qualification guard was relaxed.
- **69 planning tests and 11 leadership-review UI tests passed.**
- Research index, census, gap ledger, boundary matrix and workboard checks passed.
  The combined research index has 1,996 sources and 4,933 claims. The 8,391-case
  boundary report remains explicitly preparation only, with unresolved coverage.
- All three new receipt verifiers passed in the combined checkout, including
  retained external response hashes and the held failure body.

Individual focused packet checks overlap the combined suite and are not added to
these totals. Exact commands, timestamps, exits and raw output are in [validation/](validation/).
The [scope record](scope.json) pins the integration revision `e759264456e523b6a99a1c3cf44da19e31310e5d`;
[reviewed inputs](reviewed-inputs.json) retain its changed input blobs.

This pass changes research, its guards and coordination records. Runtime data,
artwork, saved campaigns and canonical qualification are unchanged. S25 Japan/7
crash diagnosis continues separately; no native build or competing campaign was
started for this review. S23, G5 and CP1 are not closed by research acceptance.

Run `python verify.py` in this folder to check receipt integrity and pinned Git
inputs offline. It does not repeat historical reading or award qualification.
