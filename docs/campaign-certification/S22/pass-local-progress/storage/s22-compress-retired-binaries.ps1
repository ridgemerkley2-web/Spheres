$ErrorActionPreference='Stop'
$evidence=(Resolve-Path $PSScriptRoot).Path
$root=Join-Path $evidence 's22-retired-binary-compression-01'
if(Test-Path -LiteralPath $root){throw 'Never overwrite a storage attempt'}
New-Item -ItemType Directory -Path $root | Out-Null
$rows=@()
$running=@(Get-CimInstance Win32_Process | Where-Object ExecutablePath | ForEach-Object {$_.ExecutablePath.ToLowerInvariant()})
foreach($name in @('s22-runtime-2774b0c5','s22-runtime-8de6f2a0','s22-runtime-9823b070','s22-runtime-992bc99a')){
 $dir=(Resolve-Path (Join-Path $evidence $name)).Path
 if(-not $dir.StartsWith($evidence+[IO.Path]::DirectorySeparatorChar,[StringComparison]::OrdinalIgnoreCase)){throw 'Outside intended evidence root'}
 $proof=(Get-Content (Join-Path $dir 'provenance.json') -Raw).ToLowerInvariant()
 foreach($file in Get-ChildItem -LiteralPath $dir -File -Filter '*.exe'){
  $path=$file.FullName
  if($running -contains $path.ToLowerInvariant()){throw "Active process uses $path"}
  if(Test-Path -LiteralPath ($path+'.gz')){throw "Archive exists: $path"}
  $hash=(Get-FileHash -LiteralPath $path -Algorithm SHA256).Hash.ToLowerInvariant()
  if(-not $proof.Contains($hash)){throw "Executable differs from recorded runtime provenance: $path"}
  $rows += [ordered]@{original=$path;archive=$path+'.gz';bytes=$file.Length;sha256=$hash;last_write_utc=$file.LastWriteTimeUtc.ToString('o');runtime_provenance=Join-Path $dir 'provenance.json'}
 }
}
[ordered]@{created_utc=[DateTime]::UtcNow.ToString('o');scope='Only completed task-owned retired runtime copies. Lossless gzip, decompressed byte length and SHA256 verified before individual raw removal. Current6ff, build target, saves and repository untouched.';free_before=(Get-PSDrive C).Free;files=$rows} | ConvertTo-Json -Depth 6 | Set-Content (Join-Path $root 'manifest.json')
$journal=Join-Path $root 'journal.jsonl'
$done=@()
try{
 foreach($row in $rows){
  $input=[IO.File]::OpenRead($row.original)
  $output=[IO.File]::Open($row.archive,[IO.FileMode]::CreateNew)
  $zip=[IO.Compression.GZipStream]::new($output,[IO.Compression.CompressionLevel]::Optimal)
  try{$input.CopyTo($zip)}finally{$zip.Dispose();$output.Dispose();$input.Dispose()}
  $arch=[IO.File]::OpenRead($row.archive)
  $unzip=[IO.Compression.GZipStream]::new($arch,[IO.Compression.CompressionMode]::Decompress)
  $digest=[Security.Cryptography.IncrementalHash]::CreateHash([Security.Cryptography.HashAlgorithmName]::SHA256)
  $buffer=[byte[]]::new(1048576);$length=0L
  try{while(($n=$unzip.Read($buffer,0,$buffer.Length)) -gt 0){$digest.AppendData($buffer,0,$n);$length+=$n};$roundtrip=[Convert]::ToHexString($digest.GetHashAndReset()).ToLowerInvariant()}finally{$digest.Dispose();$unzip.Dispose();$arch.Dispose()}
  if($length -ne $row.bytes -or $roundtrip -ne $row.sha256){throw "Roundtrip mismatch: $($row.original)"}
  if((Get-FileHash -LiteralPath $row.original).Hash.ToLowerInvariant() -ne $row.sha256){throw 'Original changed before removal'}
  $active=@(Get-CimInstance Win32_Process | Where-Object ExecutablePath | ForEach-Object {$_.ExecutablePath.ToLowerInvariant()})
  if($active -contains $row.original.ToLowerInvariant()){throw 'Executable became active; retain original'}
  $record=[ordered]@{utc=[DateTime]::UtcNow.ToString('o');original=$row.original;archive=$row.archive;raw_sha256=$roundtrip;raw_bytes=$length;archive_sha256=(Get-FileHash -LiteralPath $row.archive).Hash.ToLowerInvariant();archive_bytes=(Get-Item -LiteralPath $row.archive).Length;verified=$true;removed=$false}
  $record | ConvertTo-Json -Compress | Add-Content -LiteralPath $journal
  Remove-Item -LiteralPath $row.original
  $record.removed=$true
  $record | ConvertTo-Json -Compress | Add-Content -LiteralPath $journal
  $done += $record
 }
 [ordered]@{completed_utc=[DateTime]::UtcNow.ToString('o');passed=$true;files=$done;free_after=(Get-PSDrive C).Free} | ConvertTo-Json -Depth 6 | Set-Content (Join-Path $root 'result.json')
 "Archived and round-trip verified $($done.Count) retired task binaries."
}catch{
 [ordered]@{completed_utc=[DateTime]::UtcNow.ToString('o');passed=$false;error=$_.Exception.Message;completed=$done;free_after=(Get-PSDrive C).Free} | ConvertTo-Json -Depth 6 | Set-Content (Join-Path $root 'result.json')
 throw
}
