# Task-owned one-shot cleanup of the 37 explicitly reviewed S22 preflight raw copies.
# Canonical campaign files, gzip copies and all other evidence are read-only.
Set-StrictMode -Version Latest
$ErrorActionPreference='Stop'
$base=[IO.Path]::GetFullPath((Join-Path $PSScriptRoot '..'))
$manifestPath=Join-Path $PSScriptRoot 'restore-manifest.json'
$journalPath=Join-Path $PSScriptRoot 'completion-journal.jsonl'
$resultPath=Join-Path $PSScriptRoot 'result.json'
foreach($p in @($manifestPath,$journalPath,$resultPath)){if(Test-Path -LiteralPath $p){throw "Refusing to reuse existing cleanup evidence: $p"}}
$started=[DateTime]::UtcNow.ToString('o')
$freeBefore=([IO.DriveInfo]::new([IO.Path]::GetPathRoot($base))).AvailableFreeSpace
$rows=[Collections.Generic.List[object]]::new()
$removed=[Collections.Generic.List[object]]::new()
$journal=$null
function Hash-File([string]$p){(Get-FileHash -LiteralPath $p -Algorithm SHA256).Hash.ToLowerInvariant()}
function Expanded-Hash([string]$p){
    $sourceStream=[IO.File]::OpenRead($p)
    $gzip=[IO.Compression.GZipStream]::new($sourceStream,[IO.Compression.CompressionMode]::Decompress)
    $hash=[Security.Cryptography.IncrementalHash]::CreateHash([Security.Cryptography.HashAlgorithmName]::SHA256)
    $buffer=[byte[]]::new(1048576);$bytes=0L
    try{while(($n=$gzip.Read($buffer,0,$buffer.Length)) -gt 0){$hash.AppendData($buffer,0,$n);$bytes+=$n};[pscustomobject]@{bytes=$bytes;sha256=[Convert]::ToHexString($hash.GetHashAndReset()).ToLowerInvariant()}}
    finally{$hash.Dispose();$gzip.Dispose();$sourceStream.Dispose()}
}
function Write-Exclusive([string]$p,$value){
    $stream=[IO.File]::Open($p,[IO.FileMode]::CreateNew,[IO.FileAccess]::Write,[IO.FileShare]::Read)
    $writer=[IO.StreamWriter]::new($stream,[Text.UTF8Encoding]::new($false))
    try{$writer.WriteLine(($value|ConvertTo-Json -Depth 20));$writer.Flush();$stream.Flush($true)}finally{$writer.Dispose()}
}
function Journal($value){$journal.WriteLine(($value|ConvertTo-Json -Depth 12 -Compress));$journal.Flush();$journal.BaseStream.Flush($true)}
function Confined-File([string]$p,[string]$root){
    $absolute=[IO.Path]::GetFullPath($p);$allowed=[IO.Path]::GetFullPath($root)
    if(-not $absolute.StartsWith($allowed+[IO.Path]::DirectorySeparatorChar,[StringComparison]::OrdinalIgnoreCase)){throw "Target outside explicit completed preflight root: $absolute"}
    $item=Get-Item -LiteralPath $absolute -Force
    if($item.PSIsContainer){throw "Not a single file: $absolute"}
    $at=$item
    while($null -ne $at){
        if(($at.Attributes -band [IO.FileAttributes]::ReparsePoint) -ne 0){throw "Refusing reparse point: $($at.FullName)"}
        if($at.FullName.Equals($allowed,[StringComparison]::OrdinalIgnoreCase)){break}
        $at=if($at -is [IO.FileInfo]){$at.Directory}else{$at.Parent}
    }
    if($null -eq $at){throw "Target ancestry did not reach explicit root: $absolute"}
    if(-not (Resolve-Path -LiteralPath $absolute).Path.Equals($absolute,[StringComparison]::OrdinalIgnoreCase)){throw "Resolved target identity changed: $absolute"}
    $item
}
function Receipt([string]$root,[string]$kind){
    $p=Join-Path $root $(if($kind -eq 'native'){'runner-result.json'}else{'result.json'})
    $j=Get-Content -LiteralPath $p -Raw|ConvertFrom-Json
    if(-not $j.finished_utc){throw "Incomplete preflight: $root"}
    if($kind -eq 'native'){
        if($j.mode -ne 'measure' -or $null -eq $j.exit_code){throw "Not a completed native measure: $root"}
    }elseif($j.PSObject.Properties['qualification'] -and $j.qualification -eq $true){throw "Qualification output excluded: $root"}
    [pscustomobject]@{path=$p;sha256=(Hash-File $p);completed_utc=$j.finished_utc}
}
function Add-Candidate([string]$rootRelative,[string]$slot,[string]$kind,[string]$sourceRelative,[string]$method,[string]$expected){
    $root=Join-Path $base $rootRelative
    $target=Join-Path $root $(if($kind -eq 'native'){'input.json'}else{"server/saves/$slot.json"})
    $item=Confined-File $target $root
    $source=[IO.Path]::GetFullPath((Join-Path $base $sourceRelative))
    if($target.Equals($source,[StringComparison]::OrdinalIgnoreCase)){throw 'Source equals disposable target'}
    $receipt=Receipt $root $kind
    $timestamps=[ordered]@{creation_utc=$item.CreationTimeUtc.ToString('o');last_write_utc=$item.LastWriteTimeUtc.ToString('o');last_access_utc=$item.LastAccessTimeUtc.ToString('o')}
    $rawHash=Hash-File $target;$sourceHash=Hash-File $source
    $expanded=if($method -eq 'gunzip'){Expanded-Hash $source}else{[pscustomobject]@{bytes=(Get-Item -LiteralPath $source).Length;sha256=$sourceHash}}
    if($rawHash -ne $expanded.sha256 -or $item.Length -ne $expanded.bytes -or ($expected -and $rawHash -ne $expected)){throw "Restoration mismatch before ANY deletion: $target"}
    if((Hash-File $source) -ne $sourceHash){throw "Restore source changed during validation: $source"}
    $rows.Add([ordered]@{target_absolute=$item.FullName;allowed_completed_root=[IO.Path]::GetFullPath($root);kind=$kind;slot=$slot;bytes=$item.Length;sha256=$rawHash;original_timestamps=$timestamps;completed_receipt=$receipt;restore=[ordered]@{method=$method;source_absolute=$source;source_bytes=(Get-Item -LiteralPath $source).Length;source_sha256=$sourceHash;expanded_bytes=$expanded.bytes;expanded_sha256=$expanded.sha256};reason='Disposable raw copy from an explicitly named completed, nonqualification S22 preflight; exact bytes independently restorable from retained canonical input or existing gzip.'})
}
function Reverify($r){
    $item=Confined-File $r.target_absolute $r.allowed_completed_root
    if($item.Length -ne $r.bytes -or (Hash-File $r.target_absolute) -ne $r.sha256 -or $item.LastWriteTimeUtc.ToString('o') -ne $r.original_timestamps.last_write_utc){throw "Target changed: $($r.target_absolute)"}
    if((Get-Item -LiteralPath $r.restore.source_absolute).Length -ne $r.restore.source_bytes -or (Hash-File $r.restore.source_absolute) -ne $r.restore.source_sha256){throw "Retained source changed: $($r.restore.source_absolute)"}
    $receipt=Receipt $r.allowed_completed_root $r.kind
    if($receipt.sha256 -ne $r.completed_receipt.sha256){throw "Completed receipt changed: $($receipt.path)"}
}
function Restore-Removed($r){
    if(Test-Path -LiteralPath $r.target_absolute){
        if((Hash-File $r.target_absolute) -ne $r.sha256){throw "Refusing to overwrite changed rollback target: $($r.target_absolute)"}
    }elseif($r.restore.method -eq 'copy'){
        Copy-Item -LiteralPath $r.restore.source_absolute -Destination $r.target_absolute -ErrorAction Stop
    }else{
        $sourceStream=[IO.File]::OpenRead($r.restore.source_absolute)
        $gzip=[IO.Compression.GZipStream]::new($sourceStream,[IO.Compression.CompressionMode]::Decompress)
        $output=[IO.File]::Open($r.target_absolute,[IO.FileMode]::CreateNew,[IO.FileAccess]::Write,[IO.FileShare]::None)
        try{$gzip.CopyTo($output);$output.Flush($true)}finally{$output.Dispose();$gzip.Dispose();$sourceStream.Dispose()}
    }
    if((Hash-File $r.target_absolute) -ne $r.sha256){throw 'Rollback byte verification failed'}
    $item=Get-Item -LiteralPath $r.target_absolute
    $item.CreationTimeUtc=[DateTime]::Parse($r.original_timestamps.creation_utc)
    $item.LastWriteTimeUtc=[DateTime]::Parse($r.original_timestamps.last_write_utc)
    $item.LastAccessTimeUtc=[DateTime]::Parse($r.original_timestamps.last_access_utc)
    Journal ([ordered]@{event='restored_after_failure';utc=[DateTime]::UtcNow.ToString('o');target=$r.target_absolute;sha256=$r.sha256})
}
$failure=$null;$rollbackFailures=[Collections.Generic.List[string]]::new()
try{
    $adopted='s22-preparation-active-01/campaigns/saves/s22-adopted-input.json';$aHash='fc094d539a52273a7861f942bbd04fb6ddfa42effacbf21edf1632a5fab24f27'
    $original='s19-flight-funded-recovery/france-t9dIxt/server/saves/s19-flight-qualified.json';$oHash='1af599ec1568a5b2148354de25cb800780d8ce69d352664f0a944242fac5ca8c'
    foreach($name in @('s22-native-preflight-2006-isolated-01','s22-native-preflight-2006-loadfix-01','s22-native-preflight-2006-ministry-isolated-01','s22-native-preflight-2006-stockfix-isolated-01')){Add-Candidate $name 'input' 'native' 's22-preparation-active-01/campaigns/saves/s22-2006-01-01.json' 'copy' '7de5c6d0cb30506f9fa6024e6025d31649246e1d53c138d82d0482f447726325'}
    foreach($name in @('s22-native-preflight-2015-idle-import-isolated-01','s22-native-preflight-2015-ministry-isolated-01')){Add-Candidate $name 'input' 'native' 's22-preparation-active-02/campaigns/saves/s22-2015-01-01.json' 'copy' 'e786ffb67a26c6ae28bd917d5d8dc26f857e441dd6a58d60adc1a1107937ca11'}
    Add-Candidate 's22-native-preflight-2035-idle-import-isolated-01' 'input' 'native' 's22-preparation-active-03/campaigns/saves/s22-2035-11-30.json' 'copy' '67aadca2f55280abc0e4ad97944a654ec7d173ffb65cb7d71dd59090e10503cb'
    foreach($name in @('s22-native-preflight-early-01','s22-native-preflight-early-isolated-01')){Add-Candidate $name 'input' 'native' $adopted 'copy' $aHash}
    $browsers=@(
        @('s22-browser-guards-preflight/browser-06mwgM','s22-input,s22-before','adopted'),
        @('s22-browser-guards-preflight/browser-3FEH58','s22-input,s22-before','adopted'),
        @('s22-browser-guards-preflight/browser-LP4hVW','s22-input,s22-before,s22-after','adopted'),
        @('s22-browser-map-guards-preflight/browser-Mm2QB7','s22-input,s22-before,s22-after','adopted'),
        @('s22-browser-map-roundtrip-preflight/browser-3VKqAe','s22-input,s22-before,s22-after','adopted'),
        @('s22-browser-map-roundtrip-preflight/browser-BaJ828','s22-input','adopted'),
        @('s22-browser-preflight/browser-3Svnul','s22-input,s22-before,s22-after','adopted'),
        @('s22-browser-preflight/browser-ebQ8t2','s22-input,s22-before,s22-after','adopted'),
        @('s22-browser-preflight/browser-18KktL','s22-input,s22-before','original'),
        @('s22-browser-preflight/browser-kVBuhT','s22-input,s22-before,s22-after','original'),
        @('s22-browser-preflight/browser-Sp0bs8','s22-input,s22-before,s22-after','original')
    )
    foreach($entry in $browsers){foreach($slot in $entry[1].Split(',')){
        if($slot -eq 's22-input'){
            $source=if($entry[2] -eq 'adopted'){$adopted}else{$original};$sha=if($entry[2] -eq 'adopted'){$aHash}else{$oHash}
            Add-Candidate $entry[0] $slot 'browser' $source 'copy' $sha
        }else{Add-Candidate $entry[0] $slot 'browser' ($entry[0]+'/'+$slot+'.json.gz') 'gunzip' ''}
    }}
    $total=($rows|Measure-Object -Property bytes -Sum).Sum
    if($rows.Count -ne 37 -or $total -ne 2753814871){throw "Explicit inventory mismatch: $($rows.Count) files / $total bytes"}
    # A full second validation pass must succeed before the first removal.
    foreach($r in $rows){Reverify $r}
    Write-Exclusive $manifestPath ([ordered]@{schema='spheres-s22-raw-copy-restore/v1';status='all_37_verified_before_removal';created_utc=[DateTime]::UtcNow.ToString('o');base=$base;count=$rows.Count;raw_bytes=$total;free_bytes_before=$freeBefore;script_sha256=(Hash-File $PSCommandPath);entries=$rows;restoration_instructions=@('For method copy, copy source_absolute byte-for-byte to target_absolute. For method gunzip, decompress source_absolute directly into target_absolute; never parse/reserialize JSON.','Verify restored byte count and SHA256 equal bytes/sha256, then restore original creation/last-write/last-access UTC timestamps.','Before restoring, verify source_bytes/source_sha256. Gzip expanded_bytes/expanded_sha256 record a verified exact round trip. Refuse to overwrite an unrelated or changed target.','Canonical sources and gzip files must remain retained. This manifest and the completion journal are metadata, not replacements for those sources.')})
    $journal=[IO.StreamWriter]::new([IO.File]::Open($journalPath,[IO.FileMode]::CreateNew,[IO.FileAccess]::Write,[IO.FileShare]::Read),[Text.UTF8Encoding]::new($false))
    foreach($r in $rows){
        Reverify $r
        Journal ([ordered]@{event='verified_immediately_before_removal';utc=[DateTime]::UtcNow.ToString('o');target=$r.target_absolute;sha256=$r.sha256;source_sha256=$r.restore.source_sha256})
        # Single explicit file only; no wildcard, recursion, other shell or computed directory removal.
        Remove-Item -LiteralPath $r.target_absolute -ErrorAction Stop
        $removed.Add($r)
        if(Test-Path -LiteralPath $r.target_absolute){throw "Raw copy remains after removal: $($r.target_absolute)"}
        Journal ([ordered]@{event='removed_verified_duplicate';utc=[DateTime]::UtcNow.ToString('o');target=$r.target_absolute;bytes=$r.bytes;sha256=$r.sha256})
    }
}catch{
    $failure=$_.Exception.ToString()
    # If a target/source unexpectedly changes after the initial all-or-nothing
    # verification, restore earlier removals to keep a zero-deletion failure.
    for($i=$removed.Count-1;$i -ge 0;$i--){try{Restore-Removed $removed[$i]}catch{$rollbackFailures.Add($_.Exception.ToString())}}
}finally{if($null -ne $journal){$journal.Dispose()}}
$result=[ordered]@{schema='spheres-s22-raw-copy-cleanup/v1';started_utc=$started;finished_utc=[DateTime]::UtcNow.ToString('o');passed=($null -eq $failure);expected_count=37;verified_count=$rows.Count;removed_count=$(if($null -eq $failure){$removed.Count}else{0});removed_raw_bytes=$(if($null -eq $failure){($removed|Measure-Object bytes -Sum).Sum}else{0});manifest=$manifestPath;journal=$journalPath;free_bytes_before=$freeBefore;free_bytes_after=([IO.DriveInfo]::new([IO.Path]::GetPathRoot($base))).AvailableFreeSpace;failure=$failure;rollback_failures=$rollbackFailures}
Write-Exclusive $resultPath $result
$result|ConvertTo-Json -Depth 6
if($failure -or $rollbackFailures.Count){exit 1}
