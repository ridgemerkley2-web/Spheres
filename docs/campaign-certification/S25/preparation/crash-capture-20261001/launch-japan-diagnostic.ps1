param(
    [Parameter(Mandatory=$true)][string]$OutputDirectory,
    [switch]$Run
)
$ErrorActionPreference = 'Stop'
# One diagnostic only. Without -Run this script validates pins and prints the plan.
# Source, request, horizon and executable are deliberately not configurable.
$root = 'D:\spheres-offload\codex-next-20260928'
$binary = Join-Path $root 'lazy-digest-validation-20260930\spheres-web-test.exe'
$tool = Join-Path $root 's25-crash-prep-raw-20261001\procdump\procdump64.exe'
$request = Join-Path $root 'full-matrix-local-20260930-01\cells\japan-7\request.json'
$binaryHash = '7c0a07733fa7079c1ee2d21691be6a89b0c26e2a977e44b2356b0ff491562bf8'
$toolHash = 'd1fc99ae304bd1d2bf28abeb62531da959e2431916194981b88c958fd713a8e6'
$requestHash = '283ffcbf56774b69abc6af7bdc1d561d7cbd9af0dbc868c331e7d85d6f3b746e'
$out = [IO.Path]::GetFullPath($OutputDirectory)
if (-not $out.StartsWith($root + '\', [StringComparison]::OrdinalIgnoreCase)) { throw 'Output must be a new descendant of the declared D: task root' }
if ($out -match '\s|["`]') { throw 'Use an output path without spaces, quotes or backticks for the native argument vector' }
if (Test-Path -LiteralPath $out) { throw 'Output already exists; preserve it and select a new directory' }
for ($ancestor = [IO.DirectoryInfo]::new($out).Parent; $null -ne $ancestor; $ancestor = $ancestor.Parent) {
    if ($ancestor.Exists -and ($ancestor.Attributes -band [IO.FileAttributes]::ReparsePoint)) { throw 'Output ancestors must not contain a junction or symbolic link' }
}
function Assert-Pin([string]$Path, [string]$Hash) {
    if ((Get-FileHash -LiteralPath $Path -Algorithm SHA256).Hash.ToLowerInvariant() -ne $Hash) { throw "Changed pinned file: $Path" }
}
Assert-Pin $binary $binaryHash
Assert-Pin $tool $toolHash
Assert-Pin $request $requestHash
$signature = Get-AuthenticodeSignature -LiteralPath $tool
if ($signature.Status -ne 'Valid' -or $signature.SignerCertificate.Subject -notlike 'CN=Microsoft Corporation,*') { throw 'Official tool signature verification failed' }
$actualRequest = Get-Content -LiteralPath $request -Raw | ConvertFrom-Json
if ($actualRequest.format -ne 'spheres-stability-cell/v1' -or $actualRequest.id -ne 'japan-7' -or $actualRequest.country -ne 'Japan' -or $actualRequest.seed -ne 7 -or $actualRequest.through -ne '2035-12-31' -or $actualRequest.revision -ne '68ba0622ec709b78617aadd1f9198d18f532bb32') { throw 'Request identity mismatch' }
$native = Join-Path $out 'native'
$dumps = Join-Path $out 'dumps'
$newRequest = Join-Path $out 'request.json'
$dumpInspector = Join-Path $PSScriptRoot 'inspect_dump.py'
$dumpInspectorHash = (Get-FileHash -LiteralPath $dumpInspector -Algorithm SHA256).Hash.ToLowerInvariant()
$frozenDumpInspector = Join-Path $out 'inspect_dump.py'
$toolArgs = @('-ma','-e','-n','1','-x',$dumps,$binary,'--exact','s25_stability_tests::s25_stability_cell','--ignored','--nocapture','--test-threads=1')
$plan = [ordered]@{
    format='spheres-s25-native-diagnostic/v1'; candidate_revision=$actualRequest.revision;
    diagnostic_only=$true; qualification=$false; s25_complete=$false; campaign_pass=$false;
    executable=$binary; binary_sha256=$binaryHash; tool=$tool; tool_sha256=$toolHash;
    request_source=$request; request_sha256=$requestHash; request=$newRequest; cwd=$out;
    arguments=$toolArgs; environment=@{SPHERES_S25_REQUEST=$newRequest;SPHERES_S25_OUT=$native};
    limits=@{poll_seconds=2;disk_floor_bytes=3GB;dump_reserve_bytes=64GB;available_commit_start_bytes=3GB;
        available_commit_stop_bytes=1GB;max_output_bytes=128GB;max_log_bytes=64MB;max_hours=12;max_dumps=1};
    source_line_symbols_available=$false;coff_function_symbols_available=$true
    dump_inspector_sha256=$dumpInspectorHash
}
if (-not $Run) { $plan | ConvertTo-Json -Depth 8; exit 0 }
if ((Get-ItemProperty -LiteralPath 'HKCU:\Software\Sysinternals\ProcDump').EulaAccepted -ne 1) { throw 'Portable tool EULA acceptance is missing' }
if (@(Get-Process -Name cargo,rustc,rust-lld,clang,link -ErrorAction SilentlyContinue).Count) { throw 'Native build is active; coordinate a diagnostic slot first' }
function Get-Resources {
    $os = Get-CimInstance Win32_OperatingSystem
    $volumes = @(Get-CimInstance Win32_LogicalDisk -Filter "DeviceID='C:' OR DeviceID='D:'" | Select-Object DeviceID,FreeSpace,Size)
    $memory = Get-CimInstance Win32_PerfFormattedData_PerfOS_Memory
    return @{utc=(Get-Date).ToUniversalTime().ToString('o');volumes=$volumes;
        available_physical_bytes=[long]$os.FreePhysicalMemory*1024;
        commit_bytes=[long]$memory.CommittedBytes;commit_limit_bytes=[long]$memory.CommitLimit;
        available_commit_bytes=([long]$memory.CommitLimit-[long]$memory.CommittedBytes)}
}
$preflight = Get-Resources
if (@($preflight.volumes).Count -ne 2) { throw 'Both C: and D: volume checks are required' }
foreach ($v in $preflight.volumes) { if ($v.FreeSpace -lt 3GB) { throw 'Original disk safety floor is not met' } }
if (($preflight.volumes | Where-Object DeviceID -eq 'D:').FreeSpace -lt 67GB) { throw 'Insufficient D: reserve for one full dump plus original disk floor' }
if ($preflight.available_commit_bytes -lt 3GB) { throw 'Insufficient available commit for safe launch; no campaign started' }
[IO.Directory]::CreateDirectory($out) | Out-Null
# The frozen test itself creates SPHERES_S25_OUT and requires it to be absent.
[IO.Directory]::CreateDirectory($dumps) | Out-Null
[IO.File]::Copy($request,$newRequest,$false)
[IO.File]::Copy($dumpInspector,$frozenDumpInspector,$false)
Assert-Pin $frozenDumpInspector $dumpInspectorHash
$utf8 = [Text.UTF8Encoding]::new($false)
function Write-NewJson([string]$Path, $Value) {
    $stream = [IO.File]::Open($Path,[IO.FileMode]::CreateNew,[IO.FileAccess]::Write,[IO.FileShare]::Read)
    try { $bytes=$utf8.GetBytes(($Value | ConvertTo-Json -Depth 12) + "`n"); $stream.Write($bytes,0,$bytes.Length);$stream.Flush($true) } finally { $stream.Dispose() }
}
Write-NewJson (Join-Path $out 'plan.json') $plan
Write-NewJson (Join-Path $out 'preflight.json') $preflight
$journal = $null
$monitor = $null
$owned = @{}
$cleanupFailures = @()
$primary = $null
$status = 'not_started'
$started = Get-Date
$lastSize = $started
$totalBytes = 0L
function Write-Event([string]$Event, $Data) {
    $line = @{utc=(Get-Date).ToUniversalTime().ToString('o');event=$Event;data=$Data} | ConvertTo-Json -Depth 10 -Compress
    $bytes = $utf8.GetBytes($line + "`n")
    $journal.Write($bytes,0,$bytes.Length); $journal.Flush($true)
}
function Discover-OwnedChild([switch]$Cleanup) {
    if ($null -eq $monitor) { return }
    foreach ($child in @(Get-CimInstance Win32_Process -Filter "ParentProcessId=$($monitor.Id)")) {
        if ($child.ExecutablePath -ne $binary) {
            if (-not $Cleanup) { Write-Event 'uncontrolled_auxiliary_monitor_child' @{pid=$child.ProcessId;path=$child.ExecutablePath;name=$child.Name} }
            continue
        }
        if (-not $owned.ContainsKey([int]$child.ProcessId)) {
            $p = [Diagnostics.Process]::GetProcessById([int]$child.ProcessId)
            $null = $p.Handle # Hold an OS process handle; later PID reuse cannot change this target.
            $snapshotStart=([DateTimeOffset]$child.CreationDate).ToUnixTimeMilliseconds()
            $handleStart=([DateTimeOffset]$p.StartTime).ToUnixTimeMilliseconds()
            if ($snapshotStart -ne $handleStart -or $p.MainModule.FileName -ne $binary -or $p.StartTime -lt $monitor.StartTime -or ($monitor.HasExited -and $p.StartTime -gt $monitor.ExitTime)) {
                $p.Dispose();throw 'Target process identity changed between enumeration and handle acquisition'
            }
            $owned[[int]$child.ProcessId] = $p
            if (-not $Cleanup) { Write-Event 'owned_child' @{pid=$p.Id;path=$child.ExecutablePath;parent_pid=$monitor.Id;created=$child.CreationDate} }
        }
    }
}
function Get-OutputBytes {
    $sum = 0L
    $todo = [Collections.Generic.Stack[string]]::new(); $todo.Push($out)
    while ($todo.Count) {
        foreach ($item in Get-ChildItem -LiteralPath $todo.Pop() -Force) {
            if ($item.Attributes -band [IO.FileAttributes]::ReparsePoint) { throw 'Unexpected link in diagnostic outputs' }
            if ($item.PSIsContainer) { $todo.Push($item.FullName) } else { $sum += $item.Length }
        }
    }
    return $sum
}
try {
    $journal = [IO.File]::Open((Join-Path $out 'journal.jsonl'),[IO.FileMode]::CreateNew,[IO.FileAccess]::Write,[IO.FileShare]::Read)
    Write-Event 'prepared' $plan
    $env:SPHERES_S25_REQUEST = $newRequest
    $env:SPHERES_S25_OUT = $native
    # All pinned paths contain no spaces; Start-Process does not need shell quoting here.
    $monitor = Start-Process -FilePath $tool -ArgumentList $toolArgs -WorkingDirectory $out -WindowStyle Hidden -PassThru -RedirectStandardOutput (Join-Path $out 'monitor.stdout.log') -RedirectStandardError (Join-Path $out 'monitor.stderr.log')
    $null = $monitor.Handle
    Write-Event 'monitor_started' @{pid=$monitor.Id}
    $status = 'running'
    while (-not $monitor.HasExited) {
        Discover-OwnedChild
        $sample = Get-Resources
        $sample.owned_children = @($owned.Values | ForEach-Object { $_.Refresh(); if (-not $_.HasExited) { @{pid=$_.Id;private_bytes=$_.PrivateMemorySize64;working_set_bytes=$_.WorkingSet64;cpu_seconds=$_.TotalProcessorTime.TotalSeconds} } })
        Write-Event 'resources' $sample
        foreach ($v in $sample.volumes) { if ($v.FreeSpace -lt 3GB) { throw 'Resource halt: original disk floor reached' } }
        $writtenDumpBytes = [long](Get-ChildItem -LiteralPath $dumps -File | Measure-Object -Property Length -Sum).Sum
        $remainingDumpReserve = [math]::Max(0L, 64GB - $writtenDumpBytes)
        if (($sample.volumes | Where-Object DeviceID -eq 'D:').FreeSpace -lt (3GB + $remainingDumpReserve)) { throw 'Resource halt: remaining full-dump reserve reached' }
        if ($sample.available_commit_bytes -lt 1GB) { throw 'Resource halt: available commit below safety floor' }
        foreach ($name in @('monitor.stdout.log','monitor.stderr.log')) { if ((Get-Item -LiteralPath (Join-Path $out $name)).Length -gt 64MB) { throw 'Resource halt: diagnostic log bound reached' } }
        if (((Get-Date)-$started).TotalHours -ge 12) { throw 'Resource halt: diagnostic wall-clock bound reached' }
        if (((Get-Date)-$lastSize).TotalSeconds -ge 30) { $totalBytes=Get-OutputBytes;$lastSize=Get-Date;if($totalBytes -gt 128GB){throw 'Resource halt: diagnostic output bound reached'} }
        Start-Sleep -Seconds 2
        $monitor.Refresh()
    }
    # Dump-count completion can precede child termination. Never leave our run behind.
    foreach ($p in $owned.Values) { if (-not $p.WaitForExit(15000)) { throw 'Owned target remained alive after monitor exit' } }
    if ($owned.Count -ne 1) { throw 'Did not retain exactly one owned target process handle' }
    foreach ($p in $owned.Values) { if ($p.ExitCode -ne 0) { throw "Owned native target failed with exit $($p.ExitCode)" } }
    if ($monitor.ExitCode -ne 0) { throw "Capture monitor failed with exit $($monitor.ExitCode)" }
    $status = 'diagnostic_finished'
    Write-Event 'monitor_finished' @{exit_code=$monitor.ExitCode;children=@($owned.Values | ForEach-Object {@{pid=$_.Id;exit_code=$_.ExitCode}})}
} catch {
    $primary = @{message=$_.Exception.Message;type=$_.Exception.GetType().FullName;stack=$_.ScriptStackTrace;status=$status}
    $status = 'diagnostic_failed_or_resource_halted'
} finally {
    # Try every known owned handle even if one termination fails. Never target a name.
    if ($null -ne $monitor) {
        # Stop the owned spawner BEFORE the final sweep. A child created before
        # this stop remains discoverable by parent PID + exact path/lifetime.
        try { if (-not $monitor.HasExited) { $monitor.Kill();if(-not $monitor.WaitForExit(15000)){throw 'Owned monitor did not stop'} } } catch { $cleanupFailures += "Owned monitor: $($_.Exception)" }
        try { Discover-OwnedChild -Cleanup } catch { $cleanupFailures += $_.Exception.ToString() }
    }
    foreach ($p in $owned.Values) {
        try { if (-not $p.HasExited) { $p.Kill(); if (-not $p.WaitForExit(15000)) {throw 'Owned child failed to exit'} } } catch { $cleanupFailures += "Owned child $($p.Id): $($_.Exception)" }
    }
    if ($null -ne $journal) {
        try {$journal.Flush($true)} catch {$cleanupFailures += "Journal flush: $($_.Exception)"}
        try {$journal.Dispose()} catch {$cleanupFailures += "Journal dispose: $($_.Exception)"}
    }
}
$receipt = @{format='spheres-s25-native-diagnostic-result/v1';status=$status;qualification=$false;s25_complete=$false;campaign_pass=$false;
    started_utc=$started.ToUniversalTime().ToString('o');finished_utc=(Get-Date).ToUniversalTime().ToString('o');primary_error=$primary;cleanup_errors=$cleanupFailures;
    monitor_exit_code=$(if($null -ne $monitor -and $monitor.HasExited){$monitor.ExitCode}else{$null});owned_pids=@($owned.Keys);
    native_exits=@($owned.Values | ForEach-Object {@{pid=$_.Id;exited=$_.HasExited;exit_code=$(if($_.HasExited){$_.ExitCode}else{$null})}})}
if ($cleanupFailures.Count) { $receipt.status='diagnostic_failed_or_cleanup_incomplete' }
try { Assert-Pin $binary $binaryHash;Assert-Pin $tool $toolHash;Assert-Pin $newRequest $requestHash;Assert-Pin $frozenDumpInspector $dumpInspectorHash;$receipt.pins_unchanged=$true } catch {$receipt.pins_unchanged=$false;$receipt.pin_error=$_.Exception.ToString()}
$receipt.dump_inventory_errors=@()
$receipt.dumps=@()
try { $dumpFiles=@(Get-ChildItem -LiteralPath $dumps -File -Filter '*.dmp') } catch { $dumpFiles=@();$receipt.dump_inventory_errors += $_.Exception.ToString() }
foreach ($dumpFile in $dumpFiles) {
    $dumpRecord=@{path=$dumpFile.FullName;bytes=$dumpFile.Length;complete=$false}
    try {
    $dumpRecord.sha256=(Get-FileHash -LiteralPath $dumpFile.FullName -Algorithm SHA256).Hash.ToLowerInvariant()
    $inspection=Join-Path $out ($dumpFile.Name + '.inspection.json')
    Assert-Pin $frozenDumpInspector $dumpInspectorHash
    & 'C:\Users\ridge\.cache\codex-runtimes\codex-primary-runtime\dependencies\python\python.exe' $frozenDumpInspector $dumpFile.FullName $inspection > (Join-Path $out ($dumpFile.Name + '.inspection.log')) 2>&1
    if ($LASTEXITCODE -eq 0) {
        $parsed=Get-Content -LiteralPath $inspection -Raw | ConvertFrom-Json
        $dumpRecord.complete=$parsed.full_memory -and $parsed.memory64_payload_complete -and $owned.ContainsKey([int]$parsed.process_id)
        $dumpRecord.exception=$parsed.exception;$dumpRecord.inspection=$inspection
    }
    } catch { $dumpRecord.inventory_error=$_.Exception.ToString();$receipt.dump_inventory_errors += $dumpRecord.inventory_error }
    $receipt.dumps += $dumpRecord
}
if ($receipt.dump_inventory_errors.Count -or @($receipt.dumps | Where-Object {-not $_.complete}).Count) { $receipt.status='diagnostic_dump_incomplete';$receipt.dump_error='Dump inventory, structural completeness or owned PID validation failed; original primary error is retained separately' }
Write-NewJson (Join-Path $out 'result.json') $receipt
$receipt | ConvertTo-Json -Depth 8
if ($null -ne $primary -or $cleanupFailures.Count -or -not $receipt.pins_unchanged -or $receipt.dump_error) {exit 1}
