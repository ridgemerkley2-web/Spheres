# d770 supplier qualification bindings

Read-only audit of the committed runners and source; no commands below were executed. Candidate: `d770592aeead51f1313d507edd26b02d75a69bba`.

## Setup gates

1. Wait for the current Windows native qualification to complete successfully and produce `evidence/S08-final-native-route-pool-1.json`. Bind the exporter and feature profile to that proof's `binary` and `binary_sha256`, not to a guessed test executable name or an earlier `S08-final-native.json`.
2. The normal server executable was still dated 2026-09-11 at review time: `integration-target/release/spheres-web.exe`, 353,372,331 bytes. The native test lane builds the test executable, not the browser server. Run the clean candidate `binary` lane before the supplier browser. `integrated.verifyBuild` correctly rejects a stale embedded server revision.
3. `run-s08-final.py supplier-browser` inherits supplier environment variables. It supplies only Chrome, `SPHERES_BINARY`, and `SPHERES_EXPECTED_REVISION`. Clear stale external-server and audit override settings, then bind the fresh ready/preparation saves and proof explicitly. The inner browser runner records provenance but does not independently validate its supplied export proof against the candidate; the checks below supply that binding.
4. The browser input is the fresh `*-ready-before-purchase.campaign.json`; the feature acceptance input is the original qualified `*-purchased.campaign.json`. These are different phases. Do not substitute the delivered save, a browser evidence save, or an old exporter archive.
5. Node, local Playwright 1.58.2 (`integration/tools/ui/node_modules/playwright`), and installed Chrome were present. `python` resolves through the WindowsApps launcher, which earlier harness work used successfully; bind `SPHERES_AUDIT_PYTHON` explicitly to the chosen launcher. The export/performance runners require Python with `hashlib.file_digest` (3.11+). Performance is Windows-only and must run without concurrent builds, simulations, exporter, or browser work.

All labels below were unused when reviewed. If a launch has since created an evidence path, retain it and choose a new unique label. Do not overwrite prior attempts.

## 1. Bind the completed candidate test binary and export from a fresh game

Run from PowerShell after the qualification gate above:

```powershell
$runBase = 'C:\Users\ridge\Documents\Codex\2026-09-05\pick-up-the-spheres-game-on\work\campaign-certification'
$pin = 'd770592aeead51f1313d507edd26b02d75a69bba'
$python = (Get-Command python).Source
$nativeProofPath = Join-Path $runBase 'evidence/S08-final-native-route-pool-1.json'
$nativeProof = Get-Content -LiteralPath $nativeProofPath -Raw | ConvertFrom-Json
if (!$nativeProof.passed -or !$nativeProof.clean_after -or
    $nativeProof.revision_before -ne $pin -or $nativeProof.revision_after -ne $pin) {
    throw 'The new candidate native qualification has not passed cleanly.'
}
$testBinary = $nativeProof.binary
$testSha = $nativeProof.binary_sha256
& $python (Join-Path $runBase 'run-s08-supplier-export.py') $pin $testBinary $testSha 'route-pool-1'
if ($LASTEXITCODE -ne 0) { throw 'Fresh supplier export failed; retain its evidence.' }
```

The exporter sanitizes inherited `SPHERES_PROFILE*`, `SPHERES_S08_*`, and S02–S04 UI fixture variables. Its only added environment binding is:

```text
SPHERES_S08_SUPPLIER_EXPORT = BASE/evidence/S08-genuine-supplier-route-pool-1/export
```

Exact child command: the bound test executable, `equipment_view::company_view_tests::s08_fresh_tonga_paid_supplier_market_export --ignored --exact --nocapture --test-threads=1`, with `integration` as working directory. The native test starts a fresh Tonga game, uses the declared paid policy, advances at most 3,000 real days, and requires genuine French APC arrival. The wrapper checks candidate/source cleanliness, embedded revision, executable identity/hash, exact one-test completion, required phases, and unchanged source/binary. It does not launch a server or build.

## 2. Select original fresh phases, build the candidate server, then run the full browser

```powershell
$exportProofPath = Join-Path $runBase 'evidence/S08-genuine-supplier-route-pool-1/runner-result.json'
$exportProof = Get-Content -LiteralPath $exportProofPath -Raw | ConvertFrom-Json
if (!$exportProof.passed -or !$exportProof.integrity_passed -or
    $exportProof.candidate -ne $pin -or
    $exportProof.test_binary_sha256_before -ne $testSha -or
    $exportProof.test_binary_sha256_after -ne $testSha) {
    throw 'Fresh export proof does not bind this candidate and test executable.'
}
$ready = @($exportProof.archive_phases | Where-Object { $_.file.EndsWith('-ready-before-purchase.campaign.json') })
$purchased = @($exportProof.archive_phases | Where-Object { $_.file.EndsWith('-purchased.campaign.json') })
$preparation = @($exportProof.archive_phases | Where-Object { $_.file -eq '0360-progress.campaign.json' })
if ($ready.Count -ne 1 -or $purchased.Count -ne 1 -or $preparation.Count -ne 1) {
    throw 'Expected unique ready, purchased, and paid preparation phases.'
}
$readySave = Join-Path $exportProof.export_directory $ready[0].file
$purchasedSave = Join-Path $exportProof.export_directory $purchased[0].file
$preparationSave = Join-Path $exportProof.export_directory $preparation[0].file
foreach ($phase in @($ready[0], $purchased[0], $preparation[0])) {
    $file = Join-Path $exportProof.export_directory $phase.file
    if ((Get-FileHash -LiteralPath $file -Algorithm SHA256).Hash.ToLowerInvariant() -ne $phase.sha256) {
        throw ('Export archive changed: ' + $file)
    }
}

& $python (Join-Path $runBase 'run-s08-final.py') $pin 'binary' 'binary-route-pool-1'
if ($LASTEXITCODE -ne 0) { throw 'Candidate server build failed.' }
$binaryProof = Get-Content -LiteralPath (Join-Path $runBase 'evidence/S08-final-binary-route-pool-1.json') -Raw | ConvertFrom-Json
if (!$binaryProof.passed -or $binaryProof.revision_after -ne $pin) { throw 'Wrong server build proof.' }

foreach ($key in @('SPHERES_SUPPLIER_URL', 'SPHERES_SUPPLIER_SERVER_ROOT',
    'SPHERES_SUPPLIER_ARCHIVE_AUDIT', 'SPHERES_SUPPLIER_OFFER')) {
    Remove-Item -LiteralPath ('Env:' + $key) -ErrorAction SilentlyContinue
}
$env:SPHERES_SUPPLIER_SAVE = $readySave
$env:SPHERES_SUPPLIER_PREPARATION_SAVE = $preparationSave
$env:SPHERES_SUPPLIER_PROVENANCE = $exportProofPath
$env:SPHERES_SUPPLIER_OUTPUT = Join-Path $runBase 'evidence/S08-supplier-browser-route-pool-1'
$env:SPHERES_SUPPLIER_SLOT = 's08-earned-stock'
$env:SPHERES_SUPPLIER_MAX_DAYS = '120'
$env:SPHERES_SUPPLIER_ARCHIVE_AUDIT = Join-Path $runBase 'integration/tools/ui/supplier-archive-audit.cjs'
$env:SPHERES_AUDIT_PYTHON = $python
$deal = $exportProof.native_outcome.contract
if ($deal.seller -ne 'France' -or $deal.source_revision.spec.platform -ne 'ground_apc') {
    throw 'The genuine export did not establish the required French APC supplier.'
}
$env:SPHERES_SUPPLIER_OFFER = 'supplier:{0}:{1}:equipment:{2}' -f $deal.seller, $deal.company, $deal.product
& $python (Join-Path $runBase 'run-s08-final.py') $pin 'supplier-browser' 'supplier-browser-route-pool-1'
if ($LASTEXITCODE -ne 0) { throw 'Supplier browser failed; retain its inner and outer results.' }
```

`run-s08-final.py` adds/overrides:

```text
CARGO_TARGET_DIR = BASE/integration-target
SPHERES_BROWSER_CHANNEL = chrome
SPHERES_BINARY = BASE/integration-target/release/spheres-web.exe
SPHERES_EXPECTED_REVISION = d770592aeead51f1313d507edd26b02d75a69bba
```

The command is `node tools/ui/ci-supplier-imports.cjs`, working directory `integration`. It launches a disposable server, copies input archives unchanged, verifies the embedded revision and committed UI asset bytes, inspects real preparation, holds one real preview across a real date change, purchases the selected French stock, saves/loads during paid transit, waits for delivery, approves the native positive maintenance recommendation, verifies actual settled defense payment, saves/loads again, and uses Continue. Exactly two command posts remain required: purchase and separate maintenance approval. Full typed canonical world comparisons and the single maintenance-plan exception remain in force. Source and binary preservation are checked by the outer lane.

Retain both `evidence/S08-final-supplier-browser-route-pool-1.{json,log}` and the complete inner `supplier-*` directory under the explicit browser output. The custom outside-repo output is intentional; an evidence collector that only scans the default repository artifact path must also receive this directory explicitly. Record the explicit environment bindings beside those results; the outer final runner does not serialize inherited supplier environment variables.

## 3. Feature performance acceptance on the original qualified purchased phase

After all other builds, simulations, export work, and browser/server processes have finished:

```powershell
& $python (Join-Path $runBase 'run-s08-feature-performance.py') `
    $pin $testBinary $testSha $purchasedSave $exportProofPath 'route-pool-1'
if ($LASTEXITCODE -ne 0) { throw 'Declared feature acceptance failed; retain the measured result.' }
```

The runner requires the purchased path to be the actual original file in the successful same-candidate, same-test-binary export directory and checks its recorded SHA256. It copies that archive and executable into a new evidence folder, sanitizes profile/S08 exporter variables, and binds only:

```text
SPHERES_S08_PERFORMANCE_INPUT = BASE/evidence/S08-feature-performance-route-pool-1/purchased.campaign.json
SPHERES_S08_PERFORMANCE_OUT = BASE/evidence/S08-feature-performance-route-pool-1/profile.json
```

Exact child command: the copied test executable, `performance::s08_supplier_import_profile --ignored --exact --nocapture --test-threads=1`, working directory `integration`. It measures exactly 31 actual daily advances, normal state/history response construction, separate equipment-board reads, and purchase quotes. Fixed bars remain simulation p95 ≤300 ms, whole-turn p95 ≤400 ms, whole-turn max ≤750 ms, equipment p95 ≤300 ms, quote p95 ≤250 ms. Memory is observed for this feature case without an invented acceptance bar. The wrapper verifies original/copied bytes, export proof, plan, candidate, and final source integrity. A failed bar is a retained failure, not permission to rerun unchanged for a favorable sample.

No fresh export, browser, or feature acceptance result is claimed by this note. Existing e618 results do not qualify d770.
