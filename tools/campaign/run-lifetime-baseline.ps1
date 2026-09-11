#requires -Version 7.0
<#
.SYNOPSIS
Runs the already-built lifetime test against copied checkpoints, without a server or Cargo.
.DESCRIPTION
EvidenceRoot must be new and outside FixtureRoot. Parent environment variables are
never changed: profile options are sanitized only in the hidden child process.
Memory is sampled at a nominal 100 ms interval, not a guaranteed peak detector.
#>
[CmdletBinding()]
param(
    [Parameter(Mandatory)][ValidateNotNullOrEmpty()][string]$TestBinary,
    [Parameter(Mandatory)][ValidateNotNullOrEmpty()][string]$FixtureRoot,
    [Parameter(Mandatory)][ValidateNotNullOrEmpty()][string]$EvidenceRoot,
    [Parameter(Mandatory)][ValidateNotNullOrEmpty()][string]$CandidateRevision
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$failures = [Collections.Generic.List[string]]::new()
$fixtures = [Collections.Generic.List[object]]::new()
$watch = [Diagnostics.Stopwatch]::new()
$process = $null
$childStarted = $false
$evidenceCreated = $false
$stdoutFile = $stderrFile = $sampleWriter = $stdoutCopy = $stderrCopy = $null
$memory = [ordered]@{
    nominal_interval_ms = 100; samples = 0; failed_samples = 0
    max_sampled_private_bytes = $null; max_sampled_working_set_bytes = $null
    os_peak_working_set_bytes = $null; max_observed_sample_gap_ms = $null
    limits = 'Samples cover only this test process and may miss short peaks. OS peak working set is the largest OS high-water reading observed while accessible; its final value may be unavailable, and it is not private-memory peak. Timing includes process startup and profiler work; no throughput inference is made.'
}
$result = [ordered]@{
    schema_version = 1; candidate_revision = $CandidateRevision
    started_utc = [DateTime]::UtcNow.ToString('o'); finished_utc = $null
    test_binary = $null; test_binary_sha256 = $null; fixture_root = $null
    evidence_root = $null; arguments = @('performance::campaign_lifetime_profile', '--ignored', '--exact', '--nocapture', '--test-threads=1')
    child_profile_environment = $null; exit_code = $null; wall_duration_seconds = $null
    profile_json = $null; memory = $memory; fixtures = $fixtures
    passed = $false; failures = $failures
}

function Get-CheckpointHash([string]$Path) {
    if (-not (Test-Path -LiteralPath $Path -PathType Leaf)) { throw "Missing checkpoint: $Path" }
    (Get-FileHash -LiteralPath $Path -Algorithm SHA256).Hash.ToLowerInvariant()
}

function Record-MemorySample {
    try {
        $process.Refresh()
        $private = $process.PrivateMemorySize64
        $working = $process.WorkingSet64
        $peak = $process.PeakWorkingSet64
        if ($peak -gt 0 -and ($null -eq $memory.os_peak_working_set_bytes -or $peak -gt $memory.os_peak_working_set_bytes)) {
            $memory.os_peak_working_set_bytes = $peak
        }
        # Exited processes can report zero counters; do not call that a valid sample.
        if ($private -le 0 -or $working -le 0) { $memory.failed_samples++; return }
        $elapsed = $watch.Elapsed.TotalMilliseconds
        if ($null -ne $script:lastSampleMs) {
            $gap = $elapsed - $script:lastSampleMs
            if ($null -eq $memory.max_observed_sample_gap_ms -or $gap -gt $memory.max_observed_sample_gap_ms) {
                $memory.max_observed_sample_gap_ms = [Math]::Round($gap, 3)
            }
        }
        $script:lastSampleMs = $elapsed
        foreach ($entry in @(@('max_sampled_private_bytes', $private), @('max_sampled_working_set_bytes', $working))) {
            if ($null -eq $memory[$entry[0]] -or $entry[1] -gt $memory[$entry[0]]) { $memory[$entry[0]] = $entry[1] }
        }
        $memory.samples++
        $peakCell = if ($peak -gt 0) { $peak.ToString() } else { '' }
        $sampleWriter.WriteLine('{0},{1},{2},{3}', $elapsed.ToString('F3', [Globalization.CultureInfo]::InvariantCulture), $private, $working, $peakCell)
    } catch { $memory.failed_samples++ }
}

try {
    if ($CandidateRevision -notmatch '^[0-9a-f]{40}$') { throw 'CandidateRevision must be the full pinned Git SHA.' }
    if (-not (Test-Path -LiteralPath $TestBinary -PathType Leaf)) { throw "Test binary does not exist: $TestBinary" }
    if (-not (Test-Path -LiteralPath $FixtureRoot -PathType Container)) { throw "Fixture root does not exist: $FixtureRoot" }
    $binaryPath = (Resolve-Path -LiteralPath $TestBinary).ProviderPath
    $inputPath = (Resolve-Path -LiteralPath $FixtureRoot).ProviderPath
    $outputPath = [IO.Path]::GetFullPath($ExecutionContext.SessionState.Path.GetUnresolvedProviderPathFromPSPath($EvidenceRoot))
    if (Test-Path -LiteralPath $outputPath) { throw "Evidence already exists; choose a new folder: $outputPath" }
    $inputPrefix = $inputPath.TrimEnd([IO.Path]::DirectorySeparatorChar, [IO.Path]::AltDirectorySeparatorChar) + [IO.Path]::DirectorySeparatorChar
    if ($outputPath.StartsWith($inputPrefix, [StringComparison]::OrdinalIgnoreCase)) {
        throw 'EvidenceRoot must be outside the copied fixture root.'
    }
    $result.test_binary = $binaryPath
    $result.test_binary_sha256 = (Get-FileHash -LiteralPath $binaryPath -Algorithm SHA256).Hash.ToLowerInvariant()
    $result.fixture_root = $inputPath
    $result.evidence_root = $outputPath
    foreach ($age in @(0, 10, 30)) {
        $path = Join-Path $inputPath "saves/profile-idle_human-$age.json"
        $fixtures.Add([ordered]@{ checkpoint_years = $age; path = $path; sha256_before = (Get-CheckpointHash $path); sha256_after = $null; unchanged = $false })
    }
    # No -Force: a competing/existing directory must not be reused.
    New-Item -ItemType Directory -Path $outputPath -ErrorAction Stop | Out-Null
    $evidenceCreated = $true
    $result.profile_json = Join-Path $outputPath 'profile.json'
    $stdoutFile = [IO.File]::Open((Join-Path $outputPath 'stdout.log'), [IO.FileMode]::CreateNew, [IO.FileAccess]::Write, [IO.FileShare]::Read)
    $stderrFile = [IO.File]::Open((Join-Path $outputPath 'stderr.log'), [IO.FileMode]::CreateNew, [IO.FileAccess]::Write, [IO.FileShare]::Read)
    $sampleWriter = [IO.StreamWriter]::new([IO.File]::Open((Join-Path $outputPath 'memory-samples.csv'), [IO.FileMode]::CreateNew, [IO.FileAccess]::Write, [IO.FileShare]::Read))
    $sampleWriter.AutoFlush = $true
    $sampleWriter.WriteLine('elapsed_ms,private_bytes,working_set_bytes,os_peak_working_set_bytes')
    $script:lastSampleMs = $null

    $start = [Diagnostics.ProcessStartInfo]::new()
    $start.FileName = $binaryPath
    $start.WorkingDirectory = $outputPath
    $start.UseShellExecute = $false
    $start.CreateNoWindow = $true
    $start.WindowStyle = [Diagnostics.ProcessWindowStyle]::Hidden
    $start.RedirectStandardOutput = $true
    $start.RedirectStandardError = $true
    foreach ($argument in $result.arguments) { $start.ArgumentList.Add($argument) }
    # Sanitize every inherited profile option, including preparation/resume/export.
    # ProcessStartInfo owns this copy; the parent's environment needs no mutation/restoration.
    foreach ($key in @($start.Environment.Keys)) {
        if ($key.StartsWith('SPHERES_PROFILE_', [StringComparison]::OrdinalIgnoreCase)) { [void]$start.Environment.Remove($key) }
    }
    $start.Environment['SPHERES_PROFILE_INPUT'] = $inputPath
    $start.Environment['SPHERES_PROFILE_OUT'] = $result.profile_json
    $result.child_profile_environment = [ordered]@{ SPHERES_PROFILE_INPUT = $inputPath; SPHERES_PROFILE_OUT = $result.profile_json }
    $process = [Diagnostics.Process]::new()
    $process.StartInfo = $start
    $watch.Start()
    if (-not $process.Start()) { throw 'The test process did not start.' }
    $childStarted = $true
    $stdoutCopy = $process.StandardOutput.BaseStream.CopyToAsync($stdoutFile)
    $stderrCopy = $process.StandardError.BaseStream.CopyToAsync($stderrFile)
    Record-MemorySample
    while (-not $process.WaitForExit(100)) {
        if ($stdoutCopy.IsFaulted -or $stderrCopy.IsFaulted) { throw 'Log capture failed while the test was running.' }
        Record-MemorySample
    }
    $watch.Stop()
    Record-MemorySample
    $result.exit_code = $process.ExitCode
    if ($process.ExitCode -ne 0) { $failures.Add("Profile test exited with code $($process.ExitCode).") }
} catch {
    $failures.Add($_.Exception.Message)
} finally {
    if ($childStarted) {
        try {
            if (-not $process.HasExited) { $process.Kill($true); $process.WaitForExit(); $failures.Add('Test process was terminated after a runner error.') }
            $result.exit_code = $process.ExitCode
        } catch { $failures.Add("Could not finalize test process: $($_.Exception.Message)") }
    }
    $watch.Stop()
    if ($childStarted) { $result.wall_duration_seconds = [Math]::Round($watch.Elapsed.TotalSeconds, 6) }
    foreach ($copy in @($stdoutCopy, $stderrCopy)) {
        if ($null -ne $copy) { try { $copy.GetAwaiter().GetResult() } catch { $failures.Add("Log capture failed: $($_.Exception.Message)") } }
    }
    foreach ($handle in @($sampleWriter, $stdoutFile, $stderrFile, $process)) {
        if ($null -ne $handle) { try { $handle.Dispose() } catch { $failures.Add("Could not close evidence handle: $($_.Exception.Message)") } }
    }
    foreach ($fixture in $fixtures) {
        try {
            $fixture.sha256_after = Get-CheckpointHash $fixture.path
            $fixture.unchanged = $fixture.sha256_before -eq $fixture.sha256_after
            if (-not $fixture.unchanged) { $failures.Add("Checkpoint changed: $($fixture.path)") }
        } catch { $failures.Add("Checkpoint verification failed: $($_.Exception.Message)") }
    }
}

if ($childStarted -and $result.exit_code -eq 0) {
    try {
        $profileReport = Get-Content -LiteralPath $result.profile_json -Raw | ConvertFrom-Json
        if ($profileReport.mode -ne 'measure_input' -or @($profileReport.results).Count -ne 6) {
            throw 'Expected all six input measurements (two scenarios at years 0, 10 and 30).'
        }
        if ($profileReport.revision -ne $CandidateRevision.Substring(0, 12)) {
            throw "Embedded binary revision '$($profileReport.revision)' differs from the clean candidate pin."
        }
        foreach ($scenario in @('idle_human', 'industry_and_war')) {
            foreach ($age in @(0, 10, 30)) {
                if (@($profileReport.results | Where-Object { $_.scenario -eq $scenario -and $_.checkpoint_years -eq $age }).Count -ne 1) {
                    throw "Missing or duplicate measurement: $scenario year $age."
                }
                $row = $profileReport.results | Where-Object { $_.scenario -eq $scenario -and $_.checkpoint_years -eq $age }
                if ($row.whole_server_turn.samples -ne 31 -or $row.simulation_and_history_recording.samples -ne 31) {
                    throw "Expected 31 daily samples: $scenario year $age."
                }
            }
        }
    } catch { $failures.Add("Profile output validation failed: $($_.Exception.Message)") }
}
$result.finished_utc = [DateTime]::UtcNow.ToString('o')
$result.passed = $childStarted -and $result.exit_code -eq 0 -and $failures.Count -eq 0
$json = $result | ConvertTo-Json -Depth 8
if ($evidenceCreated) {
    try {
        $reportFile = [IO.StreamWriter]::new([IO.File]::Open((Join-Path $result.evidence_root 'runner-result.json'), [IO.FileMode]::CreateNew, [IO.FileAccess]::Write, [IO.FileShare]::Read))
        try { $reportFile.WriteLine($json) } finally { $reportFile.Dispose() }
    } catch {
        $failures.Add("Could not save runner-result.json: $($_.Exception.Message)")
        $result.passed = $false
        $json = $result | ConvertTo-Json -Depth 8
    }
}
Write-Output $json
if (-not $result.passed) { exit 1 }
exit 0
