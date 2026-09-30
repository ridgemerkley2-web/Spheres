$ErrorActionPreference = 'Stop'
$taskRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
$configPath = Join-Path $taskRoot 'config.json'
$prepared = Get-Content -Raw -LiteralPath (Join-Path $taskRoot 'prepared.json') | ConvertFrom-Json
$config = Get-Content -Raw -LiteralPath $configPath | ConvertFrom-Json
$observed = (Get-FileHash -LiteralPath $configPath -Algorithm SHA256).Hash.ToLowerInvariant()
if ($observed -ne $prepared.config.sha256) { throw 'Configuration changed' }
$launchDirectory = Join-Path $taskRoot 'launch'
if (Test-Path -LiteralPath $launchDirectory) { throw 'Dependent launch directory already exists; no retry or overwrite' }
$null = New-Item -ItemType Directory -Path $launchDirectory
$arguments = @('-B', '-X', 'utf8', ('"' + (Join-Path $taskRoot 'dependent_verify.py') + '"'), '--config', ('"' + $configPath + '"'), '--config-sha256', $observed)
$worker = Start-Process -FilePath $config.python -ArgumentList $arguments -WorkingDirectory $taskRoot -WindowStyle Hidden -PassThru -RedirectStandardOutput (Join-Path $launchDirectory 'worker.stdout.log') -RedirectStandardError (Join-Path $launchDirectory 'worker.stderr.log')
$metadata = Get-CimInstance Win32_Process -Filter ('ProcessId = ' + $worker.Id) | Select-Object ProcessId,ParentProcessId,CreationDate,ExecutablePath,CommandLine
$record = [ordered]@{ format='spheres-dependent-verification-launch/v1'; launched_utc=[DateTime]::UtcNow.ToString('o'); worker_pid=$worker.Id; worker=$metadata; config_sha256=$observed; exact_parent_pid=$config.parent_pid; exact_parent_creation_filetime=$config.parent_creation_filetime; window_style='Hidden'; one_off=$true; recurring=$false; campaign_launched=$false; qualification=$false; s25_complete=$false }
$receipt = Join-Path $launchDirectory 'receipt.json'
$stream = [IO.File]::Open($receipt, [IO.FileMode]::CreateNew, [IO.FileAccess]::Write, [IO.FileShare]::Read)
try {
    $bytes = [Text.UTF8Encoding]::new($false).GetBytes(($record | ConvertTo-Json -Depth 12) + [Environment]::NewLine)
    $stream.Write($bytes, 0, $bytes.Length)
    $stream.Flush($true)
} finally { $stream.Dispose() }
$deadline = [DateTime]::UtcNow.AddSeconds(20)
while (-not (Test-Path -LiteralPath (Join-Path $taskRoot 'pending.json')) -and -not (Test-Path -LiteralPath (Join-Path $taskRoot 'result.json')) -and [DateTime]::UtcNow -lt $deadline) {
    Start-Sleep -Milliseconds 250
}
if (Test-Path -LiteralPath (Join-Path $taskRoot 'result.json')) {
    Get-Content -Raw -LiteralPath (Join-Path $taskRoot 'result.json')
    throw 'Dependent job ended during launch; inspect retained records, do not retry automatically'
}
if (-not (Test-Path -LiteralPath (Join-Path $taskRoot 'pending.json'))) { throw 'No armed handshake yet; inspect owned worker without retrying' }
Get-Content -Raw -LiteralPath (Join-Path $taskRoot 'pending.json')
