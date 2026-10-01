"""One isolated resource timing invocation after coordinator grants quiet CPU."""
from datetime import datetime, timezone
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import time

OUT = Path(__file__).resolve().parent
ROOT = next(p for p in OUT.parents if (p / '.git').exists())
PACKET = OUT
if len(sys.argv) > 1:
    OUT = OUT / sys.argv[1]
    OUT.mkdir(exist_ok=False)

def sha(path):
    with path.open('rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()

def put(name, value):
    (OUT / name).write_text(json.dumps(value, indent=2) + '\n', encoding='utf-8')

build = json.loads((PACKET / 'build-02/build-result.json').read_text())
assert build['exit_code'] == 0 and build['source_pins_unchanged']
assert len(build['binaries']) == 1
binary = build['binaries'][0]
path = Path(binary['path'])
assert sha(path) == binary['sha256']
assert not (OUT / 'resource-timing.log').exists(), 'Preserve every timing attempt.'
sample = r'''
$sampleStart = Get-Date
$sampleBefore = @{}
Get-Process | ForEach-Object { if ($null -ne $_.CPU) { $sampleBefore[$_.Id] = $_.CPU } }
Start-Sleep -Seconds 3
$sampleEnd = Get-Date
$sampleElapsed = ($sampleEnd - $sampleStart).TotalSeconds
$sampleRows = @(Get-Process | ForEach-Object {
  if ($sampleBefore.ContainsKey($_.Id) -and $null -ne $_.CPU) {
    [PSCustomObject]@{ id=$_.Id; name=$_.ProcessName; cpu_seconds=[Math]::Max(0, $_.CPU-$sampleBefore[$_.Id]); working_set=$_.WorkingSet64 }
  }
} | Sort-Object cpu_seconds -Descending)
[PSCustomObject]@{ start_utc=$sampleStart.ToUniversalTime().ToString('o'); end_utc=$sampleEnd.ToUniversalTime().ToString('o'); elapsed_seconds=$sampleElapsed; logical_processors=[Environment]::ProcessorCount; processes=$sampleRows } | ConvertTo-Json -Depth 5
'''
environment = json.loads(subprocess.check_output(['powershell', '-NoProfile', '-Command', sample]))
put('timing-environment.json', environment)
busy_builds = [p for p in environment['processes'] if p['name'] in ('rustc', 'cargo', 'cc1', 'lto-wrapper', 'ld')]
assert not busy_builds, 'Wait for compilers; do not run the timing test under build load.'
if len(sys.argv) > 1:
    assert not [p for p in environment['processes'] if p['name'] == 'git' and p['cpu_seconds'] > 0.05], 'Wait for the background Git scan before the confirmation.'
command = ['cargo', 'test', '--locked', '--release', '-p', 'spheres-sim', '--lib',
           'tests::the_resource_pass_stays_under_budget', '--', '--exact', '--nocapture', '--test-threads=1']
env = dict(os.environ, CARGO_TARGET_DIR=build['target'])
started = datetime.now(timezone.utc).isoformat()
start = time.monotonic()
with (OUT / 'resource-timing.log').open('wb') as stream:
    result = subprocess.run(command, cwd=ROOT, env=env, stdout=stream, stderr=subprocess.STDOUT)
receipt = {'command': command, 'cwd': str(ROOT), 'cargo_target_dir': build['target'],
           'started_utc': started, 'finished_utc': datetime.now(timezone.utc).isoformat(),
           'elapsed_seconds': time.monotonic() - start, 'exit_code': result.returncode,
           'binary_before': binary, 'binary_sha256_after': sha(path),
           'log_sha256': sha(OUT / 'resource-timing.log'),
           'environment_sha256': sha(OUT / 'timing-environment.json'),
           'qualification': 'One standalone existing resource timing test only; no A1/outcome/CP1 acceptance.'}
receipt['binary_unchanged'] = receipt['binary_sha256_after'] == binary['sha256']
put('resource-timing-result.json', receipt)
print(json.dumps(receipt))
raise SystemExit(result.returncode or int(not receipt['binary_unchanged']))
