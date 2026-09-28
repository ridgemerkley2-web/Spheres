# S22 renderer functional preflight

**Functional preflight passed; qualification is false.** This native browser run used candidate `594c7ce1356a9c5a393e1079e4b7abff48cf093d`, with source checkout `dc73474f5c71c0d164f42042010c25dd67b9dd67`, from 2026-09-28 05:34:23.407 to 05:35:22.514 UTC. Full native workspace regression tests ran concurrently. The [recorded run scope](predeclared-run-scope.json) therefore excludes every timing result here from S22 qualification. S22 remains open.

[result.json](result.json) and [s22-renderer-result.json](s22-renderer-result.json) retain the complete original observations, source hashes and functional checks. The run used an owned headless Edge 146.0.3856.97 process and the actual native France campaign. Browser viewport was 1920 x 1080 at DPR 1, with a separate 390 px inspection check. The driver, renderer harness, buffer probe, texture probe, four screenshots, read-only before/after API states and existing compressed Chrome trace are preserved exactly.

## Functional observations

- The actual main CityMesh path respected its unchanged 8-entry / 800,000-triangle ceiling, including uploads: attributed buffer peak **86,399,352 bytes**, below 86,400,000. City revisits, Cities-Off and Low detail worked.
- Globe, Arsenal and fighter inspection contexts recovered after loss. Buffer and texture declarations cleared during loss and resources were recreated for subsequent rendering.
- Six native fighter designer visits rendered a detailed model; each closed viewer retained **zero buffers, zero buffer payload, zero textures and zero declared texture payload**. These are viewer-specific observations; the persistent globe and bounded Arsenal cache remain separate.
- The controlled below-fold card fixture deferred geometry until intersection and released removed-card references. This isolated fixture does not prove every campaign catalogue workflow.
- Both wrapper and renderer reported `memory_complete: true` for the implemented buffer/texture instrumentation. The displayed map context declared **90,969,034 texture texel bytes**; the fighter viewer declared **15,379,112**. These are API-declared mip payloads, **not measured physical VRAM**. Driver overhead, framebuffer/renderbuffer surfaces, padding and CPU images remain outside that texture counter. Original JS heap and owned-process snapshots are retained separately.
- Before/after read-only API states are byte-identical. The native saved-campaign SHA-256 stayed `0caef8f8da7e491098b65a3f59f10ac9b5164dfd9c57bf622ac9a93ff8e12633`; the route did not advance or mutate the campaign.

## Timing is diagnostic only

The fighter orbit recorded **1,199 completed draw frames in 12,000.2 ms**, or **99.9150014 FPS**, after a separate three-second settling window. Counted callbacks submitted actual draws and completed `gl.finish`; this measures instrumented draw completion, not compositor presentation. The raw sample and its local numerical `passed` value remain intact, but concurrent native regressions make it unsuitable for qualification.

The first native fighter inspection visit took **1,316.8976 ms**. Five repeat visits ranged **839.5015-933.7283 ms** (approximately 840-934 ms). These include visible room/family navigation and automation observation overhead after the game was already loaded. They are **not cold-page load times**. Native-page startup timings are separately named in the original wrapper result and carry the same diagnostic limitation.

## Existing offline and unit coverage

The [offline art audit](../preflight-art/README.md) remains recorded at `846df4797712935efb5a221e188d410b09f0d083`. Comparison against the current checkout found **38 of its 39 source pins unchanged as Git blobs and original disk SHA-256 hashes**. Only `arsenal3d.js` changed, for detached mounted/pending canvas cleanup and cache diagnostics. Geometry, scene planning, accounting, budgets, exporters and all 13 GLBs are unchanged; the full art audit was not rerun.

The historical **1,727/1,727 UI regression pass** at `bb83d3b81d5e93a80925dfaeda1b6ec8764f7906` covers unchanged current UI and focused renderer/control test blobs. This is retained coverage, **not a new test run or final-candidate qualification**. Live browser performance, final-source gates and later campaign workload measurements still require their own evidence.

## Integrity and exclusions

[manifest.json](manifest.json) records original paths, candidate/binary/source pins, exact byte lengths and SHA-256 hashes for every copied artifact. All captured Git blobs were verified against those hashes. `.gitattributes` disables text conversion. `chrome-trace.json.gz` was copied without decompression or recompression.

The disposable server directory, executable and redundant before/after campaign-save archives are excluded. Both read-only API state records and save digests remain; the immutable adopted input is retained once in [runtime-preflight/inputs](../runtime-preflight/inputs/) with uncompressed SHA-256 `fc094d539a52273a7861f942bbd04fb6ddfa42effacbf21edf1632a5fab24f27`. No captured result was edited to strengthen its scope.
