import datetime, hashlib, json, os, pathlib, re, subprocess, sys
base=pathlib.Path(__file__).resolve().parent
repo=base/'integration'
revision,lane=sys.argv[1:3]
assert re.fullmatch('[0-9a-f]{40}',revision)
label=sys.argv[3] if len(sys.argv)>3 else lane
assert re.fullmatch('[A-Za-z0-9_-]+',label)
stem=base/'evidence'/('S08-final-'+label)
def git(*args): return subprocess.check_output(['git','-c','core.longpaths=true',*args],cwd=repo,text=True).strip()
def sha(path): return hashlib.sha256(path.read_bytes()).hexdigest()
assert git('rev-parse','HEAD')==revision and not git('status','--porcelain')
assert not stem.with_suffix('.log').exists() and not stem.with_suffix('.json').exists()
env=os.environ.copy(); env['CARGO_TARGET_DIR']=str(base/'integration-target')
commands={
 'web':['cargo','test','--locked','--release','-p','spheres-web','--no-fail-fast'],
 'native':['cargo','test','--locked','--release','--workspace','--no-fail-fast'],
 'node':['node','tools/ui/run-unit.cjs'],
 'binary':['cargo','build','--locked','--release','-p','spheres-web'],
 'browser':['node','tools/ui/ci-browser.cjs'],
 'browser-installed':['node',str(base/'s05-installed-browser.cjs')],
 'money-browser':['node','tools/ui/ci-money.cjs'],
 'construction-browser':['node','tools/ui/ci-construction.cjs'],
 'supplier-browser':['node','tools/ui/ci-supplier-imports.cjs'],
 'peace':['cargo','test','--locked','--release','-p','spheres-sim','--test','s04_peace_boundaries','--','--nocapture'],
 'guidance':['node','--test','tools/ui/check_guidance_ui.cjs','tools/ui/check_guidance_host.cjs']
}
if lane.startswith('browser') or lane in ('money-browser','construction-browser','supplier-browser'):
 env['SPHERES_BROWSER_CHANNEL']='chrome'
 env['SPHERES_BINARY']=str(base/'integration-target/release/spheres-web.exe')
 env['SPHERES_EXPECTED_REVISION']=revision
command=commands[lane]
browser_lane=lane.startswith('browser') or lane in ('money-browser','construction-browser','supplier-browser')
executable=base/'integration-target/release/spheres-web.exe'
executable_before=sha(executable) if browser_lane else None
proof={'revision_before':revision,'clean_before':True,'command':command,'runner_sha256':sha(pathlib.Path(__file__)),'binary_sha256_before':executable_before,'started_utc':datetime.datetime.now(datetime.timezone.utc).isoformat()}
with stem.with_suffix('.log').open('xb') as log:
 result=subprocess.run(command,cwd=repo,env=env,stdout=log,stderr=subprocess.STDOUT,creationflags=subprocess.CREATE_NO_WINDOW if os.name=='nt' else 0)
proof.update(exit_code=result.returncode,finished_utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),revision_after=git('rev-parse','HEAD'),clean_after=not git('status','--porcelain'))
if lane in ('web','peace','native'):
 logtext=stem.with_suffix('.log').read_text(encoding='utf-8',errors='replace')
 pattern=r'Running unittests[^\n]*\(([^\n]+spheres_web-[^\n]+\.exe)\)' if lane in ('web','native') else r'Running tests[^\n]*\(([^\n]+\.exe)\)'
 matches=list(dict.fromkeys(re.findall(pattern,logtext)))
 binary=pathlib.Path(matches[0]) if len(matches)==1 else None
 if binary and not binary.is_absolute():binary=(repo/binary).resolve()
 proof['test_binary_matches']=matches
 totals=[tuple(map(int,m)) for m in re.findall(r'test result: (?:ok|FAILED)\. (\d+) passed; (\d+) failed; (\d+) ignored;',logtext)]
 proof['totals']={'passed':sum(t[0] for t in totals),'failed':sum(t[1] for t in totals),'ignored':sum(t[2] for t in totals),'completed_targets':len(totals)}
elif lane=='binary' or lane.startswith('browser') or lane in ('money-browser','construction-browser','supplier-browser'): binary=base/'integration-target/release/spheres-web.exe'
else: binary=None
if binary: proof.update(binary=str(binary),binary_sha256=sha(binary))
proof['log_sha256']=sha(stem.with_suffix('.log'))
proof['preservation_passed']=proof['revision_after']==revision and proof['clean_after'] and (executable_before is None or sha(executable)==executable_before)
proof['passed']=result.returncode==0 and proof['preservation_passed'] and (lane not in ('web','peace','native') or binary is not None and proof['totals']['completed_targets']>0 and proof['totals']['failed']==0)
stem.with_suffix('.json').write_text(json.dumps(proof,indent=2)+'\n',encoding='utf-8')
print(json.dumps(proof,indent=2),flush=True)
sys.exit(0 if proof['passed'] else (result.returncode or 1))
