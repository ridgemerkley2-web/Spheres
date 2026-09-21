"""Qualify the committed browser harness against the unchanged S16 runtime."""
import datetime, hashlib, json, os, pathlib, re, subprocess, sys
base=pathlib.Path(__file__).resolve().parent; repo=base/'integration'
runtime,label=sys.argv[1:3]
assert re.fullmatch('[0-9a-f]{40}',runtime) and re.fullmatch('[A-Za-z0-9_-]+',label)
def git(*args): return subprocess.check_output(['git','-c','core.longpaths=true',*args],cwd=repo,text=True).strip()
def sha(p):
    with open(p,'rb') as f:return hashlib.file_digest(f,'sha256').hexdigest()
def now():return datetime.datetime.now(datetime.timezone.utc).isoformat()
head=git('rev-parse','HEAD'); assert not git('status','--porcelain')
changed=git('diff','--name-only',runtime,head).splitlines()
assert changed==['tools/ui/ci-air-defense.cjs']
paths=['spheres-sim','spheres-web','Cargo.toml','Cargo.lock']
assert not git('diff','--name-only',runtime,head,'--',*paths)
stem=base/'evidence'/('S16-'+label); assert not stem.with_suffix('.log').exists()
binary=base/'integration-target/release/spheres-web.exe'
build=json.loads((base/'evidence/S16-final3-binary.json').read_text())
assert build['passed'] and build['revision']==runtime and sha(binary)==build['binary']['sha256']
fixture=base/'evidence/S16-fixture-final3'; driver=repo/'tools/ui/ci-air-defense.cjs'
env=dict(os.environ);env.update(SPHERES_BINARY=str(binary),SPHERES_EXPECTED_REVISION=runtime,
 SPHERES_S16_FIXTURE_DIR=str(fixture),SPHERES_S16_BROWSER_OUTPUT=str(base/'evidence'/('S16-browser-'+label)),SPHERES_BROWSER_CHANNEL='chrome')
record={'runtime_revision':runtime,'driver_revision':head,'changed_paths':changed,'clean_before':True,'runtime_paths':paths,
 'runtime_diff':[],'driver_sha256':sha(driver),'runner_sha256':sha(__file__),'binary_sha256':sha(binary),
 'started_utc':now(),'command':['node','tools/ui/ci-air-defense.cjs']}
with stem.with_suffix('.log').open('xb') as f:
    result=subprocess.run(record['command'],cwd=repo,env=env,stdout=f,stderr=subprocess.STDOUT,creationflags=subprocess.CREATE_NO_WINDOW)
record.update(exit_code=result.returncode,finished_utc=now(),head_after=git('rev-parse','HEAD'),clean_after=not git('status','--porcelain'),log_sha256=sha(stem.with_suffix('.log')))
record['passed']=result.returncode==0 and record['clean_after'] and record['head_after']==head and sha(binary)==record['binary_sha256'] and sha(driver)==record['driver_sha256']
if result.returncode==0:
    line=json.loads(stem.with_suffix('.log').read_text(encoding='utf8').strip().splitlines()[-1])
    p=pathlib.Path(line['result']); browser=json.loads(p.read_text())
    assert browser['passed'] and browser['source']['revision']==head and browser['build']['revision']==runtime
    assert browser['build']['binary_sha256']==record['binary_sha256'] and browser['source']['driver_sha256']==record['driver_sha256']
    record['result']={'path':str(p),'sha256':sha(p)}
stem.with_suffix('.json').write_text(json.dumps(record,indent=2)+'\n',encoding='utf8')
print(json.dumps(record,indent=2),flush=True)
sys.exit(0 if record['passed'] else 1)
