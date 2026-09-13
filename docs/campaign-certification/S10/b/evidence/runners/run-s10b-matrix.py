import datetime,hashlib,json,os,pathlib,re,subprocess,sys
base=pathlib.Path(__file__).resolve().parent;repo=base/'integration'
pin,label=sys.argv[1:3]
assert re.fullmatch('[0-9a-f]{40}',pin) and re.fullmatch('[A-Za-z0-9_-]+',label)
def git(*args):return subprocess.check_output(['git','-c','core.longpaths=true',*args],cwd=repo,text=True).strip()
def sha(p):
 with open(p,'rb') as f:return hashlib.file_digest(f,'sha256').hexdigest()
head=git('rev-parse','HEAD');assert not git('status','--porcelain')
assert not git('diff','--name-only',pin,head,'--','spheres-sim','spheres-cli','spheres-web','Cargo.toml','Cargo.lock')
exe=base/'integration-target/release/spheres-web.exe';binary_hash=sha(exe)
stem=base/'evidence'/('S10b-matrix-driver-'+label);assert not stem.with_suffix('.json').exists() and not stem.with_suffix('.log').exists()
env=os.environ.copy();env.update(SPHERES_BINARY=str(exe),SPHERES_EXPECTED_REVISION=pin,SPHERES_BROWSER_CHANNEL='chrome',SPHERES_COUNTRY_MATRIX_OUTPUT=str(base/'evidence'/('S10b-matrix-'+label)))
command=['node','tools/ui/ci-government-country-matrix.cjs']
proof={'runtime_revision':pin,'driver_revision':head,'complete_runtime_source_equal':True,'runner_sha256':sha(__file__),'driver_sha256':sha(repo/'tools/ui/ci-government-country-matrix.cjs'),'binary_sha256_before':binary_hash,'command':command,'started_utc':datetime.datetime.now(datetime.timezone.utc).isoformat()}
with stem.with_suffix('.log').open('xb') as log:
 p=subprocess.run(command,cwd=repo,env=env,stdout=log,stderr=subprocess.STDOUT,creationflags=subprocess.CREATE_NO_WINDOW)
proof.update(exit_code=p.returncode,finished_utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),revision_after=git('rev-parse','HEAD'),clean_after=not git('status','--porcelain'),binary_sha256_after=sha(exe),log_sha256=sha(stem.with_suffix('.log')))
proof['passed']=p.returncode==0 and proof['revision_after']==head and proof['clean_after'] and binary_hash==proof['binary_sha256_after']
stem.with_suffix('.json').write_text(json.dumps(proof,indent=2)+'\n',encoding='utf-8')
print(json.dumps(proof,indent=2),flush=True)
sys.exit(0 if proof['passed'] else 1)
