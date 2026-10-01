# Retained recipe for the single bounded confirmation window on 1 October.
# The existing confirmation-window.json means this checkpoint was already used.
param([string]$Python = 'python')
$confirmReceipt = Join-Path $PSScriptRoot 'confirmation-window.json'
if (Test-Path -LiteralPath $confirmReceipt) { throw 'Preserve the recorded window; a new checkpoint is required for any later measurement.' }
$confirmDeadline = (Get-Date).AddSeconds(45)
$confirmQuiet = $false
while ((Get-Date) -lt $confirmDeadline) {
    if (@(Get-Process git -ErrorAction SilentlyContinue).Count -eq 0) {
        $confirmQuiet = $true
        break
    }
    Start-Sleep -Seconds 2
}
if ($confirmQuiet) {
    & $Python (Join-Path $PSScriptRoot 'record_timing.py') confirmation-01
} else {
    $confirmResult = [PSCustomObject]@{
        status = 'confirmation_not_run'
        reason = 'App background Git scans persisted through bounded45-second wait'
        checked_utc = (Get-Date).ToUniversalTime().ToString('o')
        remaining_git = @(Get-Process git -ErrorAction SilentlyContinue | Select-Object Id,CPU)
    }
    $confirmResult | ConvertTo-Json -Depth 4 | Set-Content -LiteralPath $confirmReceipt -Encoding utf8
    $confirmResult | ConvertTo-Json -Depth 4
}
