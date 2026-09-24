param([Parameter(Mandatory=$true)][int]$Iteration,[string]$Note='Frozen source inputs before exact-binary measurement.')
$ErrorActionPreference='Stop'
$evidencePath=$PSScriptRoot
$repoPath=[IO.Path]::GetFullPath((Join-Path $evidencePath '..\..\integration'))
Set-Location $repoPath
$tracked=@(git -c core.quotepath=false ls-files)
if($LASTEXITCODE -ne 0){throw 'Cannot enumerate current tracked source inputs'}
$changed=@(git -c core.safecrlf=false diff --name-only HEAD; git ls-files --others --exclude-standard)|Sort-Object -Unique
$nativePattern='^(spheres-(sim|web|cli)/(src/|data/|tests/|.*\.rs$|Cargo\.(toml|lock)$)|spheres-web/vendor/|Cargo\.(toml|lock)$|rust-toolchain(\.toml)?$|\.cargo/)'
$nativePaths=@($tracked | Where-Object {$_ -match $nativePattern})
# Include all tracked UI assets, a conservative superset of literal native
# embeds. Hash binary assets without duplicating them in the source-body copy.
$otherInputs=@($tracked | Where-Object {$_ -match '^(spheres-web/ui/|\.gitattributes$|\.github/workflows/|tools/campaign/active-fixture-layout\.json$)'})
$literalEmbeds=@()
foreach($rustPath in @($nativePaths+$changed | Sort-Object -Unique | Where-Object {$_ -match '\.rs$'})){
 if(-not(Test-Path -LiteralPath $rustPath -PathType Leaf)){continue}
 $body=[IO.File]::ReadAllText([IO.Path]::GetFullPath($rustPath),[Text.Encoding]::UTF8)
 foreach($match in [regex]::Matches($body,'include(?:_str|_bytes)?!\s*\(\s*"([^"\r\n]+)"')){
  $full=[IO.Path]::GetFullPath((Join-Path ([IO.Path]::GetDirectoryName([IO.Path]::GetFullPath($rustPath))) $match.Groups[1].Value))
  if(-not $full.StartsWith($repoPath+[IO.Path]::DirectorySeparatorChar,[StringComparison]::OrdinalIgnoreCase)){throw "Embedded input escapes repository: $rustPath"}
  $relative=$full.Substring($repoPath.Length+1).Replace('\','/')
  if(Test-Path -LiteralPath $full -PathType Leaf){$literalEmbeds += $relative}
 }
}
$paths=@($nativePaths)+$otherInputs+$literalEmbeds+$changed|Sort-Object -Unique
$scope='Fresh current-Git enumeration of all native Rust source/data/tests/build metadata and vendored sources, all tracked web UI assets (superset of native embeds), literal include targets, active fixture layout, Git attributes/workflows, and every changed/untracked input. Raw and canonical hashes are recorded; no inherited historical path list is used.'
$sha=[Security.Cryptography.SHA256]::Create()
$utf8=[Text.UTF8Encoding]::new($false,$true)
$files=foreach($path in $paths){
 if(-not (Test-Path -LiteralPath $path -PathType Leaf)){[pscustomobject]@{path=$path;missing=$true};continue}
 $raw=[IO.File]::ReadAllBytes([IO.Path]::GetFullPath($path));$mode='binary';$normalized=$raw
 try{$s=$utf8.GetString($raw);if(-not $s.Contains([char]0)){$normalized=[Text.Encoding]::UTF8.GetBytes($s.Replace("`r`n","`n"));$mode='utf8_crlf_to_lf'}}catch{}
 [pscustomobject]@{path=$path;raw_bytes=$raw.Length;raw_sha256=([BitConverter]::ToString($sha.ComputeHash($raw))).Replace('-','').ToLowerInvariant();normalization=$mode;sha256=([BitConverter]::ToString($sha.ComputeHash($normalized))).Replace('-','').ToLowerInvariant();changed_or_untracked=($changed -contains $path)}
}
$nativeSnapshotPath=Join-Path $evidencePath "iteration-$Iteration-native-source"
if(Test-Path -LiteralPath $nativeSnapshotPath){throw 'Refusing to overwrite an existing native source snapshot'}
$nativeSnapshots=@()
foreach($file in $files){
 $slashPath=$file.path.Replace('\','/')
 if($file.missing -or ($slashPath -notmatch $nativePattern -and $slashPath -ne 'tools/campaign/active-fixture-layout.json')){continue}
 $copyPath=Join-Path $nativeSnapshotPath $file.path
 [IO.Directory]::CreateDirectory([IO.Path]::GetDirectoryName($copyPath))|Out-Null
 [IO.File]::Copy([IO.Path]::GetFullPath($file.path),$copyPath,$false)
 $copyHash=(Get-FileHash -LiteralPath $copyPath -Algorithm SHA256).Hash.ToLowerInvariant()
 if($copyHash -ne $file.raw_sha256){throw "Source changed during capture: $($file.path)"}
 $nativeSnapshots += [pscustomobject]@{path=$file.path;sha256=$copyHash}
}
[ordered]@{iteration=$Iteration;git_head=(git rev-parse HEAD);captured_utc=(Get-Date).ToUniversalTime().ToString('o');scope=$scope;note=$Note;selection=[ordered]@{tracked_native=$nativePaths.Count;tracked_ui_and_build_controls=$otherInputs.Count;literal_include_targets=@($literalEmbeds|Sort-Object -Unique).Count;changed_or_untracked=$changed.Count};native_source_snapshot="iteration-$Iteration-native-source";native_source_files=$nativeSnapshots;files=@($files);binaries=@(Get-FileHash '..\integration-target\release\deps\spheres_sim-de8a2e7790f15db6.exe','..\integration-target\release\deps\bloc_census-b35be4d742aae476.exe'|Select-Object Path,Hash)} | ConvertTo-Json -Depth 6 | Set-Content (Join-Path $evidencePath "iteration-$Iteration-source-input-manifest.json")
Write-Output "Captured $($files.Count) source inputs for iteration $Iteration"
