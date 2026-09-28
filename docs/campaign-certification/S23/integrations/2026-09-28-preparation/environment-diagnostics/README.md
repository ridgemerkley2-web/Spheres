# CNCCFP PDF extraction environment diagnosis

Reviewer: Codex `/root/s20_preflight`, 28 September 2026. This is a targeted
environment diagnosis, not another full-suite result or historical acceptance.

The original [combined failure](../validation/avatar-suite.log) is retained:
469 tests, one failure. Its bundled Python 3.12.14 uses pypdf 6.10.0. That
extractor's layout mode interleaves wrapped organization-name cells into the
decision column of this unchanged PDF. For example, page 25's
`ALLIANCE DES CENTRISTES ET INDÉPENDANTS RÉUNIONNAIS` row no longer satisfies the
existing row grammar. All annex pages have rotation zero; rotation stripping or
transferring page rotation does not address this cell-ordering problem.

The existing PATH Python 3.13.12 uses **pypdf 6.17.0**. In that environment the
exact original test passes, corroborating **all 60 names and one-based page
locators** against the independently numbered publication table. The source PDF
remains 695,861 bytes, SHA-256
`127fc6028a49240e41a204db7bcb4a6258c23280a7316af93542b920a471899d`.
No importer, test assertion, source, package or dependency installation changed.

[diagnostic.json](diagnostic.json) records the exact interpreter, extractor
version and module hashes, test command, source hashes, unchanged-input check,
timing and [raw test log](pypdf-6.17-targeted.log). The test ran against the
integration checkout; this packet was written separately in the review checkout.
The log reports one test passed. The independent full combined suite is recorded
by its own parent packet, not inferred from this focused result.

To repeat this exact check from `tools/avatars`, use an explicitly inspected
Python/pypdf environment:

```text
python -X utf8 -c "import sys,pypdf; print(sys.version); print(pypdf.__version__)"
python -X utf8 -m unittest test_import_cnccfp_census.CnccfpImportTests.test_all_sixty_pdf_absence_rows_match_the_numbered_table_and_locators -v
```

The qualified extractor for this retained result is **6.17.0**, not an untested
range of versions. A different extractor needs its own all-60-row result; do not
skip the audit, lower the count, infer CNCCFP numbers from PDF names, or replace
the original failure. Packet files are stored as raw Git bytes.
