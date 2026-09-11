import datetime, hashlib, json, os, pathlib, re, subprocess, sys
base=pathlib.Path(__file__).resolve().parent
repo=base/'integration'
revision,lane=sys.argv[1:3]
stem=base/'evidence'/('S06-final-'+(sys.argv[3] if len(sys.argv)>3 else lane))
def git(*args): return subprocess.check_output(['git',*args],cwd=repo,text=True).strip()
def sha(path): return hashlib.sha256(path.read_bytes()).hexdigest()
assert git('rev-parse','HEAD')==revision and not git('status','--porcelain')
assert not stem.with_suffix('.log').exists()
env=os.environ.copy(); env['CARGO_TARGET_DIR']=str(base/'integration-target')
commands={
 'web':['cargo','test','--locked','--release','-p','spheres-web','--no-fail-fast'],
 'native':['cargo','test','--locked','--release','--workspace','--no-fail-fast'],
 'node':['node','tools/ui/run-unit.cjs'],
 'binary':['cargo','build','--locked','--release','-p','spheres-web'],
 'browser':['node','tools/ui/ci-browser.cjs'],
 'browser-installed':['node',str(base/'s05-installed-browser.cjs')],
 'money-browser':['node','tools/ui/ci-money.cjs'],
 'peace':['cargo','test','--locked','--release','-p','spheres-sim','--test','s04_peace_boundaries','--','--nocapture'],
 'guidance':['node','--test','tools/ui/check_guidance_ui.cjs','tools/ui/check_guidance_host.cjs']
}
if lane.startswith('browser') or lane=='money-browser':
 env['SPHERES_BROWSER_CHANNEL']='chrome'
 env['SPHERES_BINARY']=str(base/'integration-target/release/spheres-web.exe')
 env['SPHERES_EXPECTED_REVISION']=revision
command=commands[lane]
proof={'revision_before':revision,'clean_before':True,'command':command,'started_utc':datetime.datetime.now(datetime.timezone.utc).isoformat()}
with stem.with_suffix('.log').open('wb') as log:
 result=subprocess.run(command,cwd=repo,env=env,stdout=log,stderr=subprocess.STDOUT)
proof.update(exit_code=result.returncode,finished_utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),revision_after=git('rev-parse','HEAD'),clean_after=not git('status','--porcelain'))
if lane in ('web','peace','native'):
 logtext=stem.with_suffix('.log').read_text(encoding='utf-8',errors='replace')
 pattern=r'Running unittests[^\n]*\(([^\n]+spheres_web-[^\n]+\.exe)\)' if lane in ('web','native') else r'Running tests[^\n]*\(([^\n]+\.exe)\)'
 match=re.search(pattern,logtext)
 binary=pathlib.Path(match.group(1)) if match else None
elif lane=='binary' or lane.startswith('browser') or lane=='money-browser': binary=base/'integration-target/release/spheres-web.exe'
else: binary=None
if binary: proof.update(binary=str(binary),binary_sha256=sha(binary))
stem.with_suffix('.json').write_text(json.dumps(proof,indent=2)+'\n',encoding='utf-8')
print(json.dumps(proof,indent=2),flush=True)
sys.exit(result.returncode)
