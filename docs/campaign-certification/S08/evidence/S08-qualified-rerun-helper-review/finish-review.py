"""Small syntax/CLI review only: never execute qualification or measurements."""
import ast, datetime, difflib, hashlib, json, pathlib, subprocess, sys
folder=pathlib.Path(__file__).resolve().parent
base=folder.parent.parent
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
files=['run-s08-external.py','accept-s08-performance.py','run-s08-linux.py']
rows=[]
patch=[]
for name in files:
    current=base/name
    before=folder/(current.stem+'.before.py')
    ast.parse(current.read_text(encoding='utf-8'))
    after=folder/(current.stem+'.after.py')
    with after.open('xb') as stream:stream.write(current.read_bytes())
    rows.append({'path':str(current),'before_sha256':sha(before),'after_sha256':sha(current),
                 'changed':before.read_bytes()!=current.read_bytes(),'syntax':'ast.parse passed'})
    patch.extend(difflib.unified_diff(before.read_text(encoding='utf-8').splitlines(True),
        current.read_text(encoding='utf-8').splitlines(True),fromfile='before/'+name,tofile='after/'+name))
with (folder/'helpers.patch').open('x',encoding='utf-8') as stream:stream.write(''.join(patch))
help_result=subprocess.run([sys.executable,str(base/'run-s08-external.py'),'--help'],capture_output=True,text=True,
    creationflags=subprocess.CREATE_NO_WINDOW if sys.platform=='win32' else 0)
assert help_result.returncode==0 and '--native-proof' in help_result.stdout
with (folder/'external-help.txt').open('x',encoding='utf-8') as stream:stream.write(help_result.stdout)
linux=(base/'run-s08-linux.py').read_text(encoding='utf-8')
assert "label=sys.argv[2] if len(sys.argv)>2 else 'final'" in linux
assert "('evidence/S08-linux-'+label)" in linux and 'out.mkdir(exist_ok=False)' in linux
assert 'core.longpaths=true' in linux and "d['head']==pin and not d['status_porcelain']" in linux
record={'captured_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),
 'executed_qualification':False,'qualification_results_modified':False,'repo_source_modified':False,
 'checks':rows,'external_cli_help_passed':True,
 'external_proof_selection':'Explicit --native-proof wins, then SPHERES_S08_NATIVE_PROOF, then original default. Full-workspace success, same clean pin and exact binary hash are required. Proof hash is rechecked after external tests.',
 'performance_protocol_selection':'Optional third positional protocol JSON; original declared plan SHA and exact original workload/bar values remain enforced. Runner candidate/binary SHA must match that selected protocol.',
 'limits':{'simulation_p95_ms':300,'whole_turn_p95_ms':400,'whole_turn_max_ms':750,'sampled_private_memory_bytes':1073741824},
 'linux_label':'Existing LABEL selects a new S08-linux-LABEL folder; exclusive creation refuses reuse. Pin and clean source are checked before and after all suites. No Linux source change.',
 'no_overwrite':'Existing external/Linux output directories and performance acceptance.json remain refused; new acceptance uses exclusive creation.',
 'commands':[
  'python ../run-s08-final.py NEW_PIN native optimized-native',
  'python ../run-s08-external.py NEW_PIN optimized --native-proof ../evidence/S08-final-optimized-native.json',
  'python ../accept-s08-performance.py NEW_PIN ../evidence/S08-performance-optimized ../evidence/S08-performance-protocol-optimized.json',
  'python3 /mnt/c/Users/ridge/Documents/Codex/2026-09-05/pick-up-the-spheres-game-on/work/campaign-certification/run-s08-linux.py NEW_PIN optimized'],
 'notes':['Commands are templates, not executed results. Linux invocation requires the established explicit Linux toolchain environment.',
          'S08 legacy acceptance bars are unchanged. The separate failed genuine supplier feature result remains open until its runtime issue is corrected and requalified.']}
with (folder/'audit.json').open('x',encoding='utf-8') as stream:json.dump(record,stream,indent=2);stream.write('\n')
print(json.dumps({'review':str(folder),'syntax_passed':True,'qualification_executed':False,'files':rows},indent=2))
