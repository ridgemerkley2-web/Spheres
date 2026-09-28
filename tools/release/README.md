# Portable release tooling

These tools package a clean, committed Windows or Linux build, then test the
extracted executable. They do not compile the game or award campaign certification.
The historical `package_windows.py` entry point now supports both platforms.
CI exercises Python 3.13, Node 22 and the repository's pinned Playwright version.

## Build and package

Start with a clean committed checkout. Build the executable from that exact
commit and fetch the locked dependencies so their license files are available:

```text
cargo build --locked --release -p spheres-web
cargo fetch --locked
python -B tools/release/package_windows.py --binary target/release/spheres-web.exe --platform windows --output artifacts/release-01 --name SPHERES-candidate
```

On Linux use `target/release/spheres-web` and `--platform linux`. The executable's
`--build-info` response must match the full checkout commit and requested platform.
The inspection command exits before creating a campaign, listener or save files.

The output contains a portable folder, its ZIP and a JSON receipt on stdout.
The ZIP preserves exact file hashes, stable ordering, fixed timestamps and modes.
Identical input binary, committed source/asset bytes and dependency licenses
produce identical package bytes; this is not a claim of reproducible compilation
across Rust versions or operating systems. ZIP entries use stored compression;
CI compresses the surrounding artifact transport without altering the ZIP.

All required source/attribution files and patched dependency licenses must be
available. Only tracked source documents and prompts are included. Existing
folder, ZIP and requested patch destinations are refused. Use a new output
directory or name for another attempt.

There is no implicit source diff against an ancient revision. Supply
`--base <commit>` only when a particular source patch is wanted; otherwise
`source/SOURCE.json` links to the exact committed source and records input hashes.
Keep complete original source/checkouts separately when an offline source archive
is required; the portable game package is not a full Git repository.

## Verify the extracted game

```text
python -B tools/release/verify_package.py artifacts/release-01/SPHERES-candidate.zip artifacts/extracted-01 --report artifacts/extraction-01.json
node tools/release/smoke_package.cjs artifacts/release-01/SPHERES-candidate.zip <full-commit> artifacts/package-smoke-01
```

The smoke command performs its own verified extraction into a new directory.
It launches only that extracted executable, checks its build identity and
embedded assets, blocks external browser requests, starts France normally,
renders an aircraft mesh and exercises named saves, an ordinary day, normal
reload and missing-primary backup recovery at desktop and narrow widths.
Logs, screenshots and restorable archive snapshots remain in its output.
`SPHERES_BROWSER_CHANNEL=msedge` selects an installed Edge browser locally;
CI installs and uses the pinned Chromium build. `SPHERES_PYTHON` can select
the Python executable used by extraction.

```text
python -B -m unittest discover -s tools/release -p "test_*.py"
node --test tools/release/test_smoke_package.cjs
```

The independent `package` CI job runs on Windows and Linux and contributes to
the required aggregate result. A green packaging lane cannot override a failed
native, UI, art or political-calibration lane. Final S28 also requires the frozen
S27 candidate and clean-machine evidence under the campaign pathway.

## Preserve campaigns when upgrading

Extract an upgrade into a new folder. Keep the previous executable and original
saves together in their old folder; test a copied save and create a new named
slot in the new release. Older builds may not understand saves written by newer
ones. Roll back with the preserved old executable and its untouched original
save. The package's `START-HERE.txt` repeats these steps for players.
