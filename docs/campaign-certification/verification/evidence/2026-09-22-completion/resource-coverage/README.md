# Bauxite reporting repair

The original resource audit found a real omission: Guinea's positive 1990
bauxite output was excluded by the district-roster crosswalk without a marker
in the shipped artifact. The exact pinned USGS workbook also identifies Sierra
Leone as a positive producer excluded by that same worksheet's crosswalk.

The repaired artifact reports Guinea's 15,800,000 and Sierra Leone's 1,430,000
metric tons in `unrostered_producers.bauxite`. Both remain outside the roster.
No mine, modeled nation, runtime production or quantity is added. A separate
metadata field explicitly limits the coverage audit to bauxite; other
commodities remain unaudited. The continuing coverage warning is retained.

The [public-domain source fixture and provenance](../../../../../../tools/resources/fixtures/README.md)
reproduce the official 67,751-byte workbook, SHA-256
`0d8ae069552229b306ce4795a29704d7eb7c859346cfb1058ecddf1e9d268214`, exactly matching
the artifact's prior source pin. The actual named 1990 column and metric-ton
unit are read; the World aggregate is excluded.

The [working-tree receipt](result.json) records commands, original/generated
artifact hashes, exact unchanged-value comparisons and raw log hashes. This
repair was made after the parent's restored21 native run completed. No native
build or browser test is claimed for the new metadata by this record.

- Original regression: seven controls passed and the missing-marker assertion
  failed, retained in `original-missing-marker.log`.
- Repaired focused checks: **8 passed, 0 failed, 0 skipped**. Negative controls
  reject altered source bytes, incompatible provenance, wrong sheet/units,
  unmapped labels and stale roster classification. Other coverage markers and
  every preexisting resource value/limitation remain unchanged.
- Source-pinned metadata reproduction passes. Both resource artifacts exactly
  match the previously staged expected bytes.
- Simulation resource validation: **63 passed**, including two complete
  byte-identical table regenerations. Only
  `meta.source_sha256.district_resources.json` changes in the simulation table.
- Raw ground-truth audit: **99 passed, 0 failed, 9 warnings**. The old missing
  marker warning is now an honest coverage-limit warning. Full geographic
  regeneration remains explicitly skipped by `--fast`.
- Workflow YAML parses; both operating systems' JavaScript/tools job now runs
  the metadata check and all eight new source-backed controls.

This does not resolve other resource vintage/location limitations or the
unavailable full historical source reconstruction. It makes no political
calibration or final campaign-certification claim.
