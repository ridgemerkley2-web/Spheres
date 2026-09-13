[CmdletBinding()]
param([switch]$Apply)

# Prepared only; do not run while builds, simulations, browser journeys or
# measurements are active. Even the collector's dry-run hashes large inputs.
$ErrorActionPreference = 'Stop'
$s08Base = Split-Path -Parent $PSScriptRoot
$s08Candidate = 'd770592aeead51f1313d507edd26b02d75a69bba'
$s08Arguments = @(
    (Join-Path $s08Base 'collect-s08-evidence.py'),
    $s08Candidate,
    '--repo', (Join-Path $s08Base 'integration'),
    '--include', (Join-Path $s08Base 'bind-s08-performance.py'),
    '--include', (Join-Path $s08Base 'assess-s08-development-profile.py'),
    '--include', (Join-Path $s08Base 'summarize-s08-diagnostic.py'),
    '--include', (Join-Path $s08Base 'verify-s08-preservation.py'),
    '--include', (Join-Path $s08Base 's08-staging'),
    '--include', (Join-Path $s08Base 'integration/artifacts/browser-ci/campaign-i6gYiW'),
    '--browser-result', (Join-Path $s08Base 'integration/artifacts/browser-money-ci/france-4kWfWi/result.json'),
    '--supplier-browser-result', (Join-Path $s08Base 'evidence/S08-supplier-browser-route-pool-1/supplier-XZ7jkY/result.json'),
    '--supplier-browser-result', (Join-Path $s08Base 'evidence/S08-supplier-browser-corrected-route-pool-1/supplier-v6VJe7/result.json'),
    '--supplier-browser-result', (Join-Path $s08Base 'evidence/S08-supplier-browser-corrected-route-pool-2/supplier-TDUttg/result.json'),
    '--include', (Join-Path $s08Base 'S08-review-launch.json'),
    '--failed-attempt', (Join-Path $s08Base 'integration/artifacts/browser-supplier-imports-ci/supplier-g630Bz/termination-result.json')
)
# The final corrected browser passed. Launch inclusion is required: the
# collector must refuse a missing review record rather than silently omit it.
if ($Apply) { $s08Arguments += '--apply' }
# Use console output; do not create a live output file inside an input root.
& python @s08Arguments
if ($LASTEXITCODE -ne 0) { throw "S08 collector exited with code $LASTEXITCODE" }
