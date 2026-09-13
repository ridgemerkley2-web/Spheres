"""Run a committed S09 browser driver against byte-identical qualified runtime source."""
import datetime,hashlib,json,os,pathlib,re,subprocess,sys
b=pathlib.Path(__file__).resolve().parent;r=b/'integration';runtime,label=sys.argv[1:3]
assert re.fullmatch('[a-f0-9]{40}',runtime) and re.fullmatch('[A-Za-z0-9_-]+',label)
stem=b/'evidence'/('S09-browser-driver-'+label);assert not stem.with_suffix('.json').exists() and not stem.with_suffix('.log').exists()
def git(*a):return subprocess.check_output(['git','-c','core.longpaths=true',*a],cwd=r,text=True).strip()
def sha(p):
 with p.open('rb') as f:return hashlib.file_digest(f,'sha256').hexdigest()
head=git('rev-parse','HEAD');assert not git('status','--porcelain')
assert not git('diff','--name-only',runtime,head,'--','spheres-sim','spheres-cli','spheres-web','Cargo.toml','Cargo.lock')
exe=b/'integration-target/release/spheres-web.exe';script=r/'tools/ui/ci-research-design.cjs';env=os.environ.copy()
env.update(SPHERES_BINARY=str(exe),SPHERES_EXPECTED_REVISION=runtime,SPHERES_BROWSER_CHANNEL='chrome',SPHERES_RESEARCH_OUTPUT=str(b/'evidence'/('S09-browser-'+label)))
proof={'runtime_revision':runtime,'driver_revision':head,'binary_sha256_before':sha(exe),'driver_sha256_before':sha(script),'runtime_source_equal':True,'started_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'command':['node','tools/ui/ci-research-design.cjs'],'runner_sha256':sha(pathlib.Path(__file__))}
with stem.with_suffix('.log').open('xb') as f:p=subprocess.run(proof['command'],cwd=r,env=env,stdout=f,stderr=subprocess.STDOUT,creationflags=subprocess.CREATE_NO_WINDOW)
proof.update(exit_code=p.returncode,driver_revision_after=git('rev-parse','HEAD'),clean_after=not git('status','--porcelain'),binary_sha256_after=sha(exe),driver_sha256_after=sha(script),log_sha256=sha(stem.with_suffix('.log')),finished_utc=datetime.datetime.now(datetime.timezone.utc).isoformat())
proof['passed']=p.returncode==0 and proof['driver_revision_after']==head and proof['clean_after'] and proof['binary_sha256_before']==proof['binary_sha256_after'] and proof['driver_sha256_before']==proof['driver_sha256_after']
with stem.with_suffix('.json').open('x',encoding='utf-8') as f:json.dump(proof,f,indent=2);f.write('\n')
print(json.dumps(proof,indent=2),flush=True);sys.exit(0 if proof['passed'] else 1)
