# Portable release engineering closeout

Bounded task: **CODEX-S28-PACKAGE-01 complete** for tooling and package smoke
proof. Canonical S28 remains planned: S27's frozen campaign candidate, complete
Windows/Linux release qualification and clean-machine evidence remain required.
No G5 or CP1 certificate is awarded by this package.

## Exact tested package

Native candidate: `a2cf1dcc3ee89281547d820d3a3b010a67ae0d49`.
Windows x86-64 executable SHA-256:
`47b9698f42e7075f4c344d38995445e845d975d19e3e96de83a01b0ac4d88cf7`.
ZIP: `SPHERES-0.6.0-Windows.zip`, 360,485,340 bytes, SHA-256:
`6e1bd2c9dccbb40ea8613519249bad9584b7cc4256fbf22868729f9c6abb32a4`.

Two separate package invocations produced identical ZIP bytes; an independent
rehash confirmed both complete files. See the [repeatability receipt](evidence/reproducibility.json).
The executable reports its full committed revision, platform and architecture
through `--build-info`, which exits before starting a game, listener or browser.

The package now verifies that identity before assigning a release label. It
refuses existing output folders, ZIPs and explicitly requested source patches;
uses deterministic archive metadata; includes the locally patched `tiny_http`
licenses; and records committed source, artwork and embedded asset hashes.
The default ancient source patch has been removed. A source diff is created
only with an explicit `--base`; the package otherwise links to its exact source
commit. It is a playable binary distribution, not a full offline source archive.

Extraction validates every member, file hash and permission before publication.
It rejects traversal, links, duplicate/case-colliding paths, missing required
attribution, changed bytes and destination collisions. It hashes written files
again before publishing the new directory. Source/license paths in package
metadata are portable; machine-specific inspection directories are omitted.

## Actual extracted-game checks

Both [smoke 01](evidence/portable-smoke-01/summary.json) and
[smoke 02](evidence/portable-smoke-02/summary.json) passed against the same ZIP.
Smoke 02 additionally pins the full extraction implementation, alongside the
CLI and browser driver. This provenance addition does not change the package.

| Check | Actual result |
|---|---|
| Verified extraction | 86 package files verified before starting its executable |
| First run | Ordinary France selection, fresh 1 January 1990 campaign |
| Offline assets | 11 explicitly requested, hash-pinned art/terrain/model resources; zero external browser requests |
| Real 3D rendering | Defensive fighter, 200,446 triangles, positive WebGL draw submissions; screenshot inspected |
| Ordinary play | One actual day to 2 January; named save and normal reload preserve complete campaign bytes |
| Recovery | Missing-primary fixture keeps the previous backup visible; explicit backup load restores complete 1 January archive |
| Layout | Desktop 1440px and narrow 390px map/recovery screenshots inspected; no horizontal panel overflow |
| Integrity and cleanup | Package files and executable unchanged; owned server stopped; no browser, console or HTTP errors |

The missing-primary fixture renames only the disposable test campaign's primary
file; its bytes remain retained. Save comparisons exclude only the terminal
wall-clock save timestamp. Each run retains four restorable native save snapshots
and its full result as gzip, with original and retained hashes in `retention.json`.
The displaced primary is exactly the retained B snapshot; it need not be stored
twice. An [independent retained-save check](evidence/retained-verification.json)
rehashes all eight decoded saves and repeats all four complete-archive pairs.
Original local run folders remain intact. The large executable/ZIP stays
in local build output and CI artifacts, rather than being checked into Git.

## Tests, CI and limits

- Native web suite: **445 passed**, 27 opt-in tests ignored; real executable
  identity and HTTP connection tests each passed separately (447 total).
  These ran on `9c9baf22`; subsequent changes are packaging/test/documentation
  tooling. The actual package above was freshly compiled from `a2cf1dcc`.
- Packaging/extraction: **18 Python tests passed**, including checkout relocation,
  identical archives, damaged extraction and deliberately incomplete attribution.
- Browser orchestration guards: **five tests passed**. The two actual browser
  runs are separate evidence, not inferred from those synthetic tests.
- An initial Windows directory-publication test hit `WinError 5`; its retained
  diagnostic excerpt is labeled as such. A bounded retry for Windows access/sharing
  errors preserves atomic no-overwrite behavior. The full rerun passed.

[Candidate CI run](https://github.com/ridgemerkley2-web/Spheres/actions/runs/36454614585)
adds independent Windows and Linux package/build/extraction/browser lanes and
includes them in the required aggregate. **Both platform package jobs passed**
on this exact source candidate in the [final observed snapshot](evidence/ci-snapshot-02.json).
The [Linux log](evidence/linux-ci-package.log) and [Windows log](evidence/windows-ci-package.log)
record their own executable and ZIP hashes, verified extraction and successful
browser runs. Different compiler/platform inputs produce different binaries;
the repeatability claim above uses the identical local input executable.
The earlier [in-progress snapshot](evidence/ci-snapshot-01.json) remains retained.
[Artifact IDs and digests](evidence/ci-artifacts.json) include their seven-day
expiry dates; their temporary availability is not permanent release publication.
The historical-source UI test now fetches its exact retained Git revision
instead of depending on a shallow checkout containing it.

This is not an all-green aggregate CI result. The existing political A1 gate still fails its unchanged coup-concentration
limit (approximately 0.586 against `< 0.5`). Packaging does not adjust that limit
or repair political calibration. S23 content, worldwide S24, the full S25 matrix,
independent human S26 sessions and the frozen release gates remain separate.

See [release commands and upgrade/rollback instructions](../../../../../tools/release/README.md).
Keep the old executable and original saves together when testing an upgrade;
older builds are not assumed to understand newly written saves.
