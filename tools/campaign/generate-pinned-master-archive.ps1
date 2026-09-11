#requires -Version 7.0
<#
.SYNOPSIS
Creates one genuine pinned-master campaign archive using its existing test binary.
.DESCRIPTION
No Cargo, server, imported input or user campaign is used. The original profiler
runs one fresh technical USA scenario for 31 days and saves its actual final date.
Its incidental timing numbers are not performance evidence for the candidate.
#>
[CmdletBinding()]
param(
    [Parameter(Mandatory)][string]$TestBinary,
    [Parameter(Mandatory)][string]$OutputRoot
)
Set-StrictMode -Version Latest
$ErrorActionPreference='Stop'
function Get-RecordCount($Value) {
    if($null -eq $Value){return 0}
    if($Value -is [System.Collections.IDictionary]){return $Value.Count}
    if($Value -is [System.Collections.ICollection]){return $Value.Count}
    throw 'Expected a record collection in the source archive.'
}
$revision='485c223f60d5ff6e46f6ae17164bf1ee3a8764d9'
$expectedHash='2bedd29560bed02e157a6953d64c45faf91ee21aeac85083aed78a3e74f4f622'
$binary=(Resolve-Path -LiteralPath $TestBinary).ProviderPath
$hash=(Get-FileHash -LiteralPath $binary -Algorithm SHA256).Hash.ToLowerInvariant()
if($hash -ne $expectedHash){throw 'The binary does not match the recorded S01 pinned-master executable.'}
$output=[IO.Path]::GetFullPath($ExecutionContext.SessionState.Path.GetUnresolvedProviderPathFromPSPath($OutputRoot))
if(Test-Path -LiteralPath $output){throw 'OutputRoot must be a new disposable directory.'}
New-Item -ItemType Directory -Path $output -ErrorAction Stop | Out-Null
$profile=Join-Path $output 'profile.json'
$arguments=@('performance::campaign_lifetime_profile','--ignored','--exact','--nocapture','--test-threads=1')
$environment=[ordered]@{SPHERES_PROFILE_OUT=$profile;SPHERES_PROFILE_YEARS='0';SPHERES_PROFILE_SCENARIO='industry_and_war'}
$record=[ordered]@{
    schema_version=1;source_revision=$revision;test_binary=$binary;test_binary_sha256=$hash
    arguments=$arguments;child_environment=$environment;started_utc=[DateTime]::UtcNow.ToString('o')
    finished_utc=$null;exit_code=$null;passed=$false;archive=$null;archive_sha256=$null
    actual_saved_date=$null;actual_rules=$null;property_counts=$null;failure=$null
    scope='One fresh pinned-master technical USA scenario, 31 days. USA is not required campaign scope. No input archive, server or Cargo. Incidental profiler timings are not candidate performance or long-run balance qualification.'
}
$process=$stdout=$stderr=$null
try {
    $start=[Diagnostics.ProcessStartInfo]::new()
    $start.FileName=$binary;$start.WorkingDirectory=$output;$start.UseShellExecute=$false
    $start.CreateNoWindow=$true;$start.WindowStyle=[Diagnostics.ProcessWindowStyle]::Hidden
    $start.RedirectStandardOutput=$true;$start.RedirectStandardError=$true
    foreach($argument in $arguments){$start.ArgumentList.Add($argument)}
    foreach($key in @($start.Environment.Keys)){
        if($key.StartsWith('SPHERES_PROFILE_',[StringComparison]::OrdinalIgnoreCase)){[void]$start.Environment.Remove($key)}
    }
    foreach($key in $environment.Keys){$start.Environment[$key]=$environment[$key]}
    $stdout=[IO.File]::Open((Join-Path $output 'stdout.log'),[IO.FileMode]::CreateNew,[IO.FileAccess]::Write,[IO.FileShare]::Read)
    $stderr=[IO.File]::Open((Join-Path $output 'stderr.log'),[IO.FileMode]::CreateNew,[IO.FileAccess]::Write,[IO.FileShare]::Read)
    $process=[Diagnostics.Process]::new();$process.StartInfo=$start
    if(-not $process.Start()){throw 'The pinned test process did not start.'}
    $outCopy=$process.StandardOutput.BaseStream.CopyToAsync($stdout)
    $errCopy=$process.StandardError.BaseStream.CopyToAsync($stderr)
    while(-not $process.WaitForExit(100)){
        if($outCopy.IsFaulted -or $errCopy.IsFaulted){throw 'Master fixture log capture failed.'}
    }
    $outCopy.GetAwaiter().GetResult();$errCopy.GetAwaiter().GetResult()
    $record.exit_code=$process.ExitCode
    if($process.ExitCode -ne 0){throw "The pinned master generator exited $($process.ExitCode)."}
    $report=Get-Content -LiteralPath $profile -Raw | ConvertFrom-Json -AsHashtable
    if($report.revision -ne $revision.Substring(0,12) -or $report.mode -ne 'continuous' -or $report.results.Count -ne 1){throw 'Unexpected profile revision or measurement scope.'}
    $row=$report.results[0]
    if($row.scenario -ne 'industry_and_war' -or $row.checkpoint_years -ne 0 -or $row.whole_server_turn.samples -ne 31){throw 'Expected exactly the year-zero 31-day industry/war fixture.'}
    $archive=Join-Path $output 'profile-campaigns/saves/profile-industry_and_war-0.json'
    if(-not(Test-Path -LiteralPath $archive -PathType Leaf)){throw 'The profiler did not produce its original post-measurement campaign archive.'}
    $record.archive=$archive
    $record.archive_sha256=(Get-FileHash -LiteralPath $archive -Algorithm SHA256).Hash.ToLowerInvariant()
    $saved=Get-Content -LiteralPath $archive -Raw | ConvertFrom-Json -AsHashtable
    if($saved.format -ne 'spheres-campaign'){throw 'Expected a genuine browser campaign envelope.'}
    $world=$saved.world
    while($world.ContainsKey('format') -and $world.ContainsKey('world')){$world=$world.world}
    $record.actual_saved_date=$saved.saved_date;$record.actual_rules=$world.rules
    # The original save omits day=1 by design.
    $savedDay=if($world.ContainsKey('day')){$world.day}else{1}
    if($world.year -ne 1990 -or $world.month -ne 2 -or $savedDay -ne 1){throw 'The saved archive is not the actual date after 31 days from 1990-01-01.'}
    $companyBook=$world['companies'] ?? @{}
    $campaignBook=$world['campaign'] ?? @{}
    $supplyBook=$world['campaign_supply'] ?? @{}
    $record.property_counts=[ordered]@{
        nations=(Get-RecordCount $world['nations']);history=(Get-RecordCount $saved['history']);log=(Get-RecordCount $saved['log'])
        contractor_roster=(Get-RecordCount $companyBook['roster']);contractor_assignments=(Get-RecordCount $companyBook['assignments'])
        conflicts=(Get-RecordCount $world['conflicts']);campaign_sectors=(Get-RecordCount $campaignBook['sectors'])
        campaign_transfers=(Get-RecordCount $campaignBook['transfers']);supply_buffers=(Get-RecordCount $supplyBook['buffers'])
        supply_cargo=(Get-RecordCount $supplyBook['cargo'])
    }
    if($record.property_counts.contractor_roster -le 0 -or $world.rules.operational_warfare -ne 1){throw 'The fresh master archive lacks its expected company/warfare capability.'}
    if((Get-FileHash -LiteralPath $binary -Algorithm SHA256).Hash.ToLowerInvariant() -ne $hash){throw 'The pinned executable changed during generation.'}
    $record.passed=$true
} catch {
    $record.failure=$_.Exception.Message
} finally {
    if($null -ne $process){
        try{if(-not $process.HasExited){$process.Kill($true);$process.WaitForExit()}}catch{}
        $process.Dispose()
    }
    if($null -ne $stdout){$stdout.Dispose()};if($null -ne $stderr){$stderr.Dispose()}
    $record.finished_utc=[DateTime]::UtcNow.ToString('o')
    $record | ConvertTo-Json -Depth 15 | Set-Content -LiteralPath (Join-Path $output 'generation.json') -Encoding utf8NoBOM
}
$record | ConvertTo-Json -Depth 15
if(-not $record.passed){exit 1}
