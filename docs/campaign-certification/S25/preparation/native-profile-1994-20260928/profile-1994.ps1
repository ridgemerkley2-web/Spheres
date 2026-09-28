$ErrorActionPreference = 'Stop'
$sourceSave = 'D:\spheres-offload\codex-next-20260928\matrix-full-ssd-02-interrupted\cells\france-1990\native\resumed\saves\monthly.json'
$binaryPath = 'C:\Users\ridge\Documents\Codex\2026-09-05\pick-up-the-spheres-game-on\work\campaign-certification\evidence\codex-next-matrix-binary\spheres-web-test.exe'
$outputRoot = 'C:\Users\ridge\Documents\Codex\2026-09-05\pick-up-the-spheres-game-on\work\campaign-certification\evidence\codex-next-native-profile-1994-01'
$expectedInput = '5480ccc296c2fe67ef2234871d1ef5fc9f098184ca1c83d885b07010a1368b20'
$expectedBinary = 'e90b8f0cb34603f5290c8a19b106a0d6dc31e3ea7ad293eb0135199fe1fd59e2'
$inputBefore = (Get-FileHash -LiteralPath $sourceSave -Algorithm SHA256).Hash.ToLowerInvariant()
$binaryBefore = (Get-FileHash -LiteralPath $binaryPath -Algorithm SHA256).Hash.ToLowerInvariant()
if ($inputBefore -ne $expectedInput -or $binaryBefore -ne $expectedBinary) { throw 'Frozen source identity differs' }
if (Test-Path -LiteralPath $outputRoot) { throw 'Diagnostic output must be new' }
New-Item -ItemType Directory -Path $outputRoot | Out-Null
$localInput = Join-Path $outputRoot 'input.json'
Copy-Item -LiteralPath $sourceSave -Destination $localInput -ErrorAction Stop
if ((Get-FileHash -LiteralPath $localInput -Algorithm SHA256).Hash.ToLowerInvariant() -ne $expectedInput) { throw 'Copied input differs' }
$env:SPHERES_S22_INPUT = $localInput
$env:SPHERES_S22_OUT = Join-Path $outputRoot 'profile.json'
$env:SPHERES_S22_DIAGNOSTIC_EXPECT_DATE = '1994-01-01'
$env:SPHERES_S22_RENEW_BUDGET = '1'
$env:SPHERES_S22_REQUIRE_CERTIFIED = '1'
Remove-Item Env:SPHERES_S22_ADOPT_COMPETITION -ErrorAction SilentlyContinue
$testArgs = @('performance::s22_daily_subsystem_diagnosis', '--ignored', '--exact', '--nocapture', '--test-threads=1')
$record = [ordered]@{
    format = 'spheres-1994-subsystem-diagnostic-execution/v1'
    source_revision = 'ae8084e853a8eb01ef4d834017af3e94c8e2cc82'
    binary = $binaryPath
    binary_sha256_before = $binaryBefore
    original_input = $sourceSave
    input = $localInput
    input_bytes = (Get-Item -LiteralPath $localInput).Length
    input_sha256_before = $inputBefore
    args = $testArgs
    started_utc = [DateTime]::UtcNow.ToString('o')
    environment = [ordered]@{
        SPHERES_S22_INPUT = $env:SPHERES_S22_INPUT
        SPHERES_S22_OUT = $env:SPHERES_S22_OUT
        SPHERES_S22_DIAGNOSTIC_EXPECT_DATE = $env:SPHERES_S22_DIAGNOSTIC_EXPECT_DATE
        SPHERES_S22_RENEW_BUDGET = '1'
        SPHERES_S22_REQUIRE_CERTIFIED = '1'
        SPHERES_S22_ADOPT_COMPETITION = $null
    }
    qualification = $false
    scope = '31-day timing diagnosis on the immutable interrupted France/1990 Jan1994 checkpoint; independent observed/native worlds. This is not a full matrix, latency qualification or single-server memory benchmark. No source or simulation policy changes.'
}
$record | ConvertTo-Json -Depth 8 | Set-Content -LiteralPath (Join-Path $outputRoot 'execution-start.json') -Encoding utf8
$samplePath = Join-Path $outputRoot 'process.csv'
'elapsed_ms,working_bytes,private_bytes,peak_working_bytes,peak_paged_bytes,total_cpu_ms' | Set-Content -LiteralPath $samplePath -Encoding ascii
$watch = [Diagnostics.Stopwatch]::StartNew()
$process = Start-Process -FilePath $binaryPath -ArgumentList $testArgs -WorkingDirectory $outputRoot -WindowStyle Hidden -PassThru -RedirectStandardOutput (Join-Path $outputRoot 'stdout.log') -RedirectStandardError (Join-Path $outputRoot 'stderr.log')
$record.pid = $process.Id
$timedOut = $false
while (-not $process.HasExited) {
    $process.Refresh()
    if ($process.HasExited) { break }
    try {
        '{0},{1},{2},{3},{4},{5}' -f $watch.Elapsed.TotalMilliseconds.ToString('F3',[Globalization.CultureInfo]::InvariantCulture),$process.WorkingSet64,$process.PrivateMemorySize64,$process.PeakWorkingSet64,$process.PeakPagedMemorySize64,$process.TotalProcessorTime.TotalMilliseconds.ToString('F3',[Globalization.CultureInfo]::InvariantCulture) | Add-Content -LiteralPath $samplePath -Encoding ascii
    } catch {
        if (-not $process.HasExited) { throw }
    }
    if ($watch.Elapsed.TotalSeconds -gt 1200) { $timedOut = $true; $process.Kill(); break }
    Start-Sleep -Milliseconds 250
}
$process.WaitForExit()
$record.finished_utc = [DateTime]::UtcNow.ToString('o')
$record.elapsed_seconds = $watch.Elapsed.TotalSeconds
$record.exit_code = $process.ExitCode
$record.timed_out = $timedOut
$record.binary_sha256_after = (Get-FileHash -LiteralPath $binaryPath -Algorithm SHA256).Hash.ToLowerInvariant()
$record.original_input_sha256_after = (Get-FileHash -LiteralPath $sourceSave -Algorithm SHA256).Hash.ToLowerInvariant()
$record.copied_input_sha256_after = (Get-FileHash -LiteralPath $localInput -Algorithm SHA256).Hash.ToLowerInvariant()
$record.inputs_unchanged = $record.binary_sha256_after -eq $expectedBinary -and $record.original_input_sha256_after -eq $expectedInput -and $record.copied_input_sha256_after -eq $expectedInput
$record | ConvertTo-Json -Depth 8 | Set-Content -LiteralPath (Join-Path $outputRoot 'execution.json') -Encoding utf8
Get-Content -LiteralPath (Join-Path $outputRoot 'stdout.log') -Tail 8
Get-Content -LiteralPath (Join-Path $outputRoot 'stderr.log') -Tail 4
$report = Get-Content -LiteralPath $env:SPHERES_S22_OUT -Raw | ConvertFrom-Json
if ($timedOut -or $process.ExitCode -ne 0 -or -not $record.inputs_unchanged -or $report.revision -ne 'ae8084e853a8' -or $report.rows.Count -ne 31 -or -not $report.passed) { throw 'Bounded diagnostic did not pass; preserve output' }
Write-Output ('Verified 31 native days; diagnostic output: ' + $outputRoot)
