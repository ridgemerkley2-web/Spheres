param([Parameter(Mandatory=$true)][string]$EvidenceDirectory)
$ErrorActionPreference='Stop'
$launcher=Join-Path $PSScriptRoot 'launch-japan-diagnostic.ps1'
$tokens=$null;$parseErrors=$null
$ast=[Management.Automation.Language.Parser]::ParseFile($launcher,[ref]$tokens,[ref]$parseErrors)
$checks=[Collections.Generic.List[object]]::new()
function Check([string]$Name,[bool]$Passed){$checks.Add(@{name=$Name;passed=$Passed});if(-not $Passed){throw "Failed: $Name"}}
Check 'PowerShell parser' ($parseErrors.Count -eq 0)
Check 'Native output remains absent for the frozen test to create' (-not $ast.Extent.Text.Contains('[IO.Directory]::CreateDirectory($native)'))
Check 'Full dump reserve credits bytes already consumed' ($ast.Extent.Text.Contains('[math]::Max(0L, 64GB - $writtenDumpBytes)') -and $ast.Extent.Text.Contains('(3GB + $remainingDumpReserve)'))
$proposed='D:\spheres-offload\codex-next-20260928\japan-7-diagnostic-20261001-01'
$before=Test-Path -LiteralPath $proposed
$plan=(& $launcher -OutputDirectory $proposed | Out-String | ConvertFrom-Json)
Check 'Dry run leaves proposed output absent' (-not $before -and -not (Test-Path -LiteralPath $proposed))
Check 'Only full-horizon Japan7 frozen request, one exact test, one dump' ($plan.candidate_revision -eq '68ba0622ec709b78617aadd1f9198d18f532bb32' -and $plan.arguments[-1] -eq '--test-threads=1' -and $plan.limits.max_dumps -eq 1)
Check 'Qualification always false' (-not $plan.qualification -and -not $plan.s25_complete -and -not $plan.campaign_pass)
foreach($case in @(
    @{name='Existing directory refused';path=$EvidenceDirectory},
    @{name='Outside declared root refused';path='D:\not-a-s25-output'},
    @{name='Ambiguous native argument path refused';path='D:\spheres-offload\codex-next-20260928\has space'}
)){
    $refused=$false
    try {& $launcher -OutputDirectory $case.path | Out-Null} catch {$refused=$true}
    Check $case.name $refused
}
# Exercise the actual durable writer and pin checker without running a game.
foreach($name in @('Write-NewJson','Assert-Pin')) {
    $node=$ast.Find({param($n) $n -is [Management.Automation.Language.FunctionDefinitionAst] -and $n.Name -eq $name},$true)
    if($null -eq $node){throw "Missing function $name"}
    Invoke-Expression $node.Extent.Text
}
$utf8=[Text.UTF8Encoding]::new($false)
$fixture=Join-Path $EvidenceDirectory 'durable-writer-fixture.json'
Write-NewJson $fixture @{evidence='owned fixture';qualification=$false}
$hash=(Get-FileHash -LiteralPath $fixture -Algorithm SHA256).Hash.ToLowerInvariant()
$refused=$false
try {Write-NewJson $fixture @{changed=$true}} catch {$refused=$true}
Check 'Durable writer refuses overwrite and preserves original bytes' ($refused -and (Get-FileHash -LiteralPath $fixture -Algorithm SHA256).Hash.ToLowerInvariant() -eq $hash)
Assert-Pin $fixture $hash
Check 'Exact file pin accepted' $true
$refused=$false
try {Assert-Pin $fixture ('0'*64)} catch {$refused=$true}
Check 'Changed file pin refused' $refused
$result=@{checks=$checks;passed=$checks.Count;failed=0;native_campaign_launched=$false;scope='Static plan, refusal paths and real durable writer/hash functions; process lifecycle/resource-halt wrapper branches are not yet integration-tested.'}
$result | ConvertTo-Json -Depth 7
