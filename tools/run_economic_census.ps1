#Requires -Version 7.0
<#
Runs a frozen census executable sequentially, with durable logs and optional
final-world snapshots. Does not build, change the simulation, touch campaigns,
overwrite prior evidence, or launch when two census processes are observed.
Cooperating runners are serialized and every run has an exclusive evidence claim.
Requires PowerShell 7: Windows PowerShell 5.1 turns ordinary native stderr
progress into terminating errors in this merged logging pipeline.
Example: ./tools/run_economic_census.ps1 -Binary <exe> -BinarySha256 <hash>
  -ArtifactDirectory <directory> -Label v5 -Days 1826 -Seeds 42,7 -WorldSnapshots
#>
[CmdletBinding()]
param(
    [Parameter(Mandatory)][string]$Binary,
    [Parameter(Mandatory)][ValidatePattern('^[0-9A-Fa-f]{64}$')][string]$BinarySha256,
    [Parameter(Mandatory)][string]$ArtifactDirectory,
    [Parameter(Mandatory)][ValidatePattern('^[A-Za-z0-9_-]+$')][string]$Label,
    [ValidateRange(1,36525)][int]$Days = 1826,
    [UInt64[]]$Seeds = @(42,7),
    [switch]$WorldSnapshots
)
$ErrorActionPreference = 'Stop'
$censusExe = (Resolve-Path -LiteralPath $Binary).Path
$censusArtifacts = (Resolve-Path -LiteralPath $ArtifactDirectory).Path
if (-not (Test-Path -LiteralPath $censusArtifacts -PathType Container)) { throw 'Artifacts must be an existing directory.' }
if ($Seeds.Count -eq 0 -or @($Seeds | Select-Object -Unique).Count -ne $Seeds.Count) { throw 'Provide distinct seeds.' }
if ([IO.Path]::GetFileName($censusExe) -ne 'economic_competition_census.exe') { throw 'Expected the census executable.' }
$runnerMutex = [Threading.Mutex]::new($false, 'Local\SpheresEconomicCensusRunner')
$ownsMutex = $false
try {
    try { $ownsMutex = $runnerMutex.WaitOne(0) }
    catch [Threading.AbandonedMutexException] { $ownsMutex = $true }
    if (-not $ownsMutex) { throw 'Another census runner owns the queue; inspect it instead of duplicating work.' }
foreach ($censusSeed in $Seeds) {
    if ((Get-FileHash -LiteralPath $censusExe -Algorithm SHA256).Hash -ne $BinarySha256) { throw 'Frozen census binary hash changed.' }
    $stem = "economic-competition-census-$Days-seed$censusSeed-$Label"
    $reportPath = Join-Path $censusArtifacts "$stem.json"
    $logPath = Join-Path $censusArtifacts "$stem.log"
    $worldPath = Join-Path $censusArtifacts "$stem.world.json"
    $claimPath = Join-Path $censusArtifacts "$stem.run.json"
    foreach ($path in @($reportPath,$logPath,$worldPath,$claimPath)) {
        if (Test-Path -LiteralPath $path) { throw "Prior evidence exists; refusing to overwrite: $path" }
    }
    $running = @(Get-CimInstance Win32_Process -Filter "Name = 'economic_competition_census.exe'")
    if ($running.Count -ge 2) { throw 'Two census processes are already running; inspect them instead of duplicating work.' }
    if ($running | Where-Object { $_.ExecutablePath -eq $censusExe }) { throw 'This frozen binary is already running; inspect that run first.' }
    $censusArgs = @([string]$Days,[string]$censusSeed,$reportPath)
    if ($WorldSnapshots) { $censusArgs += $worldPath }
    $provenance = [ordered]@{
        started_at = [DateTimeOffset]::Now.ToString('o')
        binary = $censusExe
        binary_sha256 = $BinarySha256
        arguments = $censusArgs
        shell_pid = $PID
        powershell_version = $PSVersionTable.PSVersion.ToString()
        note = 'Exclusive run claim. Completion and exit status are recorded in the accompanying log.'
    } | ConvertTo-Json -Depth 3
    # CreateNew refuses a concurrent/existing claim instead of overwriting it.
    $claim = [IO.File]::Open($claimPath,[IO.FileMode]::CreateNew,[IO.FileAccess]::Write,[IO.FileShare]::Read)
    try {
        $bytes = [Text.Encoding]::UTF8.GetBytes($provenance)
        $claim.Write($bytes,0,$bytes.Length)
    } finally { $claim.Dispose() }
    $log = [IO.File]::Open($logPath,[IO.FileMode]::CreateNew,[IO.FileAccess]::Write,[IO.FileShare]::Read)
    try {
        $bytes = [Text.Encoding]::UTF8.GetBytes($provenance + [Environment]::NewLine)
        $log.Write($bytes,0,$bytes.Length)
    } finally { $log.Dispose() }
    Write-Output "Starting $stem; provenance=$claimPath"
    & $censusExe @censusArgs 2>&1 | Tee-Object -FilePath $logPath -Append
    $runExit = $LASTEXITCODE
    "Exited $runExit at $([DateTimeOffset]::Now.ToString('o'))" | Tee-Object -FilePath $logPath -Append
    if ($runExit -ne 0) { throw "Census exited $runExit; inspect $logPath. Later seeds were not launched." }
    $result = Get-Content -Raw -LiteralPath $reportPath | ConvertFrom-Json
    if ($result.seed -ne $censusSeed -or $result.days -ne $Days -or $result.countries.Count -ne 137) {
        throw 'Completed report does not match the requested seed, horizon and 137-country roster.'
    }
    if ($WorldSnapshots -and (-not (Test-Path -LiteralPath $worldPath -PathType Leaf) -or (Get-Item -LiteralPath $worldPath).Length -eq 0)) {
        throw 'The requested final-world snapshot was not produced.'
    }
    "Verified completion of $stem; report=$reportPath" | Tee-Object -FilePath $logPath -Append
}
} finally {
    if ($ownsMutex) { $runnerMutex.ReleaseMutex() }
    $runnerMutex.Dispose()
}
