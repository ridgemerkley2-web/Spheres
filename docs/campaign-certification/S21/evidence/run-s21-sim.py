import pathlib,subprocess,sys,os,json,hashlib,datetime,re
base=pathlib.Path(__file__).resolve().parent;repo=base/'integration'
lane,label=sys.argv[1:3]
def git(*a):return subprocess.check_output(['git','-c','core.longpaths=true',*a],cwd=repo,text=True).strip()
def sha(p):
 with open(p,'rb') as f:return hashlib.file_digest(f,'sha256').hexdigest()
def now():return datetime.datetime.now(datetime.timezone.utc).isoformat()
pin=git('rev-parse','HEAD');assert not git('status','--porcelain')
start=now();prefix=base/'evidence'/('S21-'+label+'-'+lane);assert not prefix.with_suffix('.log').exists()
env=dict(os.environ,CARGO_TARGET_DIR=str(base/'integration-target'),CARGO_BUILD_JOBS='4')
fixture=base/'evidence'/('S21-fixture-'+label);exe=base/'integration-target/release/spheres-web.exe'
env.update(SPHERES_S21_FIXTURE_DIR=str(fixture),SPHERES_S21_BROWSER_OUTPUT=str(base/'evidence'/('S21-browser-'+label)),SPHERES_BINARY=str(exe),SPHERES_EXPECTED_REVISION=pin)
commands={'sim':['cargo','test','--locked','--release','-p','spheres-sim','--lib','--no-fail-fast'],'web':['cargo','test','--locked','--release','-p','spheres-web','--no-fail-fast'],'node':['node','tools/ui/run-unit.cjs'],'binary':['cargo','build','--locked','--release','-p','spheres-web'],'fixture':['cargo','test','--locked','--release','-p','spheres-web','s21_export_review_fixtures','--','--ignored','--nocapture'],'browser':['node','tools/ui/ci-campaign-journey.cjs']}
proof={'revision':pin,'lane':lane,'command':commands[lane],'started_utc':start,'clean_before':True,'runner_sha256':sha(__file__)}
if lane=='browser':proof['binary_before']=sha(exe)
with prefix.with_suffix('.log').open('xb') as f:r=subprocess.run(commands[lane],cwd=repo,env=env,stdout=f,stderr=subprocess.STDOUT,creationflags=subprocess.CREATE_NO_WINDOW)
log=prefix.with_suffix('.log').read_text(encoding='utf8',errors='replace')
proof.update(exit_code=r.returncode,finished_utc=now(),clean_after=not git('status','--porcelain'),revision_after=git('rev-parse','HEAD'),log_sha256=sha(prefix.with_suffix('.log')))
if lane in ['web','fixture','sim']:proof['counts']=re.findall(r'test result: (?:ok|FAILED)\. (\d+) passed; (\d+) failed; (\d+) ignored;',log)
if lane=='node':proof['counts']={k:int(v) for k,v in re.findall(r'# (tests|pass|fail|skipped) (\d+)',log)}
if lane in ['binary','browser']:proof['binary_sha256']=sha(exe)
proof['passed']=r.returncode==0 and proof['revision_after']==pin and proof['clean_after']
if lane=='browser':proof['passed'] &= proof['binary_before']==proof['binary_sha256']
prefix.with_suffix('.json').write_text(json.dumps(proof,indent=2)+'\n',encoding='utf8')
print(json.dumps(proof,indent=2),flush=True)
if not proof['passed']:print(log[-6000:]);sys.exit(1)
