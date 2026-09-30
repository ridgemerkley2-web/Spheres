# Local collection of completed matrix cells

`collect_cell.py` is external collection tooling for the fixed run **36474011141**, attempt **1**, candidate **5d970f6d7370baf16760585c641d81253d1c2175** and batch manifest **70a51be54a7e3100a5267ca310bf5c7f05a3c388a389b7c54904e6365c477cc8**. It contains no network, native-launch, Git, retry, cancellation or aggregate operation.

The root coordinator owns live monitoring and downloads. Wait for a completed matching cell job and its actual uploaded artifact. Retain these **raw API JSON objects** as separate files: the selected artifact object, its completed job object, and workflow-run object. A connector wrapper/list is not accepted. Retain the ZIP exactly as downloaded; do not unpack or rename internal paths. Record the actual artifact and job IDs independently from the selected API records.

From this directory, using actual IDs and existing metadata/ZIP paths:

```text
python -B collect_cell.py collect --cell france-1990 --artifact-id <actual-artifact-id> --job-id <actual-job-id> --artifact-metadata <artifact.json> --job-metadata <job.json> --run-metadata <run.json> --zip <downloaded-cell.zip>
```

Capture stdout/stderr in a new log. The helper accepts successful **and failed** completed jobs; the latter are retained as failures, never turned into passing evidence.

Before extraction it checks exact run/head/branch/attempt, workflow/repository, expected artifact/job IDs, the assigned cell, exact artifact name, API ZIP byte size/SHA256, and the already frozen bundle. It then validates every ZIP path, rejects traversal, absolute/drive/UNC/alternate-stream paths, Windows aliases, links/special entries, case collisions, file-directory collisions, encryption and existing destinations. Files are created exclusively inside fresh `shards/CELL/`; ZIP size/CRC and extracted file hashes are checked, with a source ZIP rehash afterward.

Receipts live separately in fresh `cell-receipts/CELL/`: original metadata copies and pins, collection request, per-file extraction hashes, structured inspection result, and any failure traceback. These sidecars stay outside `shards/` so the eventual full aggregate sees only its 24 declared shard directories. Partial extraction and failure evidence remain in place. The helper will not overwrite or silently retry them.

The frozen Python verifier is imported only after its hash is checked. It rehashes the retained cell records and actual gzip campaign archives without native execution. A passing selected cell additionally passes the frozen driver's complete single-shard identity check. The full plan is retained and must match; horizons and countries/seeds cannot be shortened.

- Exit **0**: this selected cell's retained evidence verifies successfully.
- Exit **1**: coherent retained evidence verifies, but the selected cell failed.
- Exit **2**: metadata, extraction, structural, hash or verification error; inspect the retained raw ZIP/logs/receipt.
- `full_matrix_passed`, `qualification`, and `s25_complete` always remain false in this helper's report.

A later local-only recheck writes a new timestamped inspection receipt without changing the shard or frozen files:

```text
python -B collect_cell.py inspect --cell france-1990
```

It verifies retained metadata pins, source ZIP and all extracted file hashes again before calling the frozen read-only verifier. A structurally incomplete first extraction cannot be resumed or promoted by this command.

Validation performed during preparation: `--help` parsed successfully; **26 small synthetic checks passed** in `collector-synthetic-01/result.json`, covering valid extraction, digest/overwrite rejection, malicious ZIP paths/types/collisions, completed-failure metadata, wrong run/head/attempt/cell/IDs/workflow rejection, and the real frozen bundle identity. They used tiny locally authored ZIPs only. No actual cell ZIP was collected or inspected, no live request was made, and no native binary was run by these checks.

The current helper hash and exact check results are in `collector-synthetic-01/result.json`. This tooling does not replace the eventual frozen aggregate verification of all 24 actual retained cells.

