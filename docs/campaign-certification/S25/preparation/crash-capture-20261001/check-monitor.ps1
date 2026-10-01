param([Parameter(Mandatory=$true)][string]$EvidenceDirectory)
$ErrorActionPreference='Stop'
$tokens=$null;$parseErrors=$null
$launcher=Join-Path $PSScriptRoot 'launch-japan-diagnostic.ps1'
$ast=[Management.Automation.Language.Parser]::ParseFile($launcher,[ref]$tokens,[ref]$parseErrors)
if($parseErrors.Count){throw 'Launcher parse error'}
$core=@($ast.FindAll({param($n) $n -is [Management.Automation.Language.TryStatementAst] -and $n.Body.Extent.Text.Contains("Write-Event 'monitor_started'")},$true))
if($core.Count -ne 1){throw 'Cannot identify exact production monitor try/catch/finally'}
$block=[scriptblock]::Create($core[0].Extent.Text)
foreach($name in @('Write-NewJson','Write-Event','Discover-OwnedChild','Get-OutputBytes')) {
    $node=$ast.Find({param($n) $n -is [Management.Automation.Language.FunctionDefinitionAst] -and $n.Name -eq $name},$true)
    Invoke-Expression $node.Extent.Text
}
Set-Item Function:\Write-RealEvent (Get-Item Function:\Write-Event).ScriptBlock
function Write-Event([string]$Event,$Data) {
    if($mode -eq 'journal-fault' -and $Event -eq 'resources' -and $owned.Count){throw [IO.IOException]::new('synthetic journal failure')}
    if($mode -eq 'early-journal-fault' -and $Event -eq 'monitor_started') {
        # Let the owned monitor spawn its target, but fail before any registration.
        $deadline=(Get-Date).AddSeconds(2)
        do { $spawned=@(Get-CimInstance Win32_Process -Filter "ParentProcessId=$($monitor.Id)" | Where-Object Name -eq 'python.exe');if($spawned.Count){break};Start-Sleep -Milliseconds 50 } while((Get-Date) -lt $deadline)
        throw [IO.IOException]::new('synthetic early journal failure')
    }
    Write-RealEvent $Event $Data
}
function Get-Resources {
    $commit=8GB
    if($mode -eq 'resource-halt' -and $owned.Count){$commit=512MB}
    return @{utc=(Get-Date).ToUniversalTime().ToString('o');volumes=@(@{DeviceID='C:';FreeSpace=100GB},@{DeviceID='D:';FreeSpace=200GB});available_commit_bytes=$commit;synthetic=$true}
}
$results=@()
$utf8=[Text.UTF8Encoding]::new($false)
foreach($mode in @('success','captured-fault','resource-halt','journal-fault','early-journal-fault')) {
    $out=Join-Path $EvidenceDirectory ('wrapper-'+$mode)
    if(Test-Path -LiteralPath $out){throw 'Refusing to overwrite synthetic evidence'}
    New-Item -ItemType Directory -Path $out | Out-Null
    $native=Join-Path $out 'native';$dumps=Join-Path $out 'dumps';New-Item -ItemType Directory -Path $dumps | Out-Null
    $newRequest=Join-Path $out 'request.json';Write-NewJson $newRequest @{synthetic=$true}
    $fixture=Join-Path $out 'owned_fixture.py'
    $code=@'
import os, pathlib, time
native=pathlib.Path(os.environ['SPHERES_S25_OUT'])
assert not native.exists(), 'Frozen native precondition: output must not exist'
native.mkdir()
(native/'started.txt').write_text(str(os.getpid()))
time.sleep(3)
'@
    if($mode -eq 'captured-fault'){$code += "`n" + [IO.File]::ReadAllText((Join-Path $PSScriptRoot 'owned_fault.py'))}
    [IO.File]::WriteAllText($fixture,$code,$utf8)
    $binary='C:\Users\ridge\.cache\codex-runtimes\codex-primary-runtime\dependencies\python\python.exe'
    $tool='D:\spheres-offload\codex-next-20260928\s25-crash-prep-raw-20261001\procdump\procdump64.exe'
    # Test fixture paths contain no spaces, matching the production path restriction.
    $toolArgs=@('-ma','-e','-n','1','-x',$dumps,$binary,$fixture)
    $plan=@{synthetic=$true;mode=$mode;campaign_launched=$false}
    $journal=$null;$monitor=$null;$owned=@{};$cleanupFailures=@();$primary=$null;$status='not_started';$started=Get-Date;$lastSize=$started;$totalBytes=0L
    . $block
    $nativeExited=@($owned.Values | ForEach-Object {@{pid=$_.Id;exited=$_.HasExited;exit_code=$(if($_.HasExited){$_.ExitCode}else{$null})}})
    $remainingTargets=@(Get-CimInstance Win32_Process -Filter "ParentProcessId=$($monitor.Id)" | Where-Object Name -eq 'python.exe')
    $r=@{mode=$mode;status=$status;primary=$primary;cleanup=$cleanupFailures;owned=$nativeExited;monitor_exited=$monitor.HasExited;monitor_exit=$monitor.ExitCode;remaining_target_pids=@($remainingTargets | ForEach-Object ProcessId);native_created_output=(Test-Path -LiteralPath (Join-Path $native 'started.txt'));dump_count=@(Get-ChildItem -LiteralPath $dumps -Filter '*.dmp').Count;production_core_sha256=(Get-FileHash -LiteralPath $launcher -Algorithm SHA256).Hash.ToLowerInvariant()}
    Write-NewJson (Join-Path $out 'synthetic-result.json') $r
    if(($mode -ne 'early-journal-fault' -and $owned.Count -ne 1) -or @($nativeExited | Where-Object {-not $_.exited}).Count -or $remainingTargets.Count -or -not $monitor.HasExited -or $cleanupFailures.Count){throw "Ownership/cleanup failed for $mode"}
    if($mode -eq 'success' -and ($null -ne $primary -or $status -ne 'diagnostic_finished' -or -not $r.native_created_output)){throw 'Success fixture failed'}
    if($mode -eq 'captured-fault' -and ($null -eq $primary -or $r.dump_count -ne 1 -or $nativeExited[0].exit_code -eq 0)){throw 'Native fault was not retained as failure'}
    if($mode -eq 'resource-halt' -and $primary.message -notlike '*available commit*'){throw 'Resource halt was not retained'}
    if($mode -eq 'journal-fault' -and $primary.message -ne 'synthetic journal failure'){throw 'Journal primary failure was masked'}
    if($mode -eq 'early-journal-fault' -and $primary.message -ne 'synthetic early journal failure'){throw 'Early journal primary failure was masked'}
    $results += $r
}
if($ast.Extent.Text.Contains('[IO.Directory]::CreateDirectory($native)')){throw 'Frozen absent-output precondition violated'}
@{passed=$results.Count;failed=0;native_campaign_launched=$false;scope='Actual production monitor try/catch/finally and helper functions; owned Python targets and synthetic resource snapshots only.';cases=$results} | ConvertTo-Json -Depth 12
