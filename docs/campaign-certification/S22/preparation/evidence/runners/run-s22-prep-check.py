import pathlib,subprocess,sys,os,json,hashlib,datetime,re
base=pathlib.Path(__file__).resolve().parent;repo=base/'integration'
lane,label=sys.argv[1:3]
def git(*a):return subprocess.check_output(['git','-c','core.longpaths=true',*a],cwd=repo,text=True).strip()
def sha(p):
 with open(p,'rb') as f:return hashlib.file_digest(f,'sha256').hexdigest()
def now():return datetime.datetime.now(datetime.timezone.utc).isoformat()
pin=git('rev-parse','HEAD');assert not git('status','--porcelain')
prefix=base/'evidence'/('S22-prep-'+label+'-'+lane);assert not prefix.with_suffix('.log').exists()
focused=['tools/ui/check_'+x+'.cjs' for x in ['mesh_accounting','webgl_measurement','equipment_shadows','equipment_model']]
commands={'focused':['node','--test',*focused],'records':['node','tools/ui/bench_art.cjs','--check-records'],
 'budget-guard':['node','tools/ui/bench_art.cjs','--check'],'manifest':['node','tools/ui/build_art_manifest.cjs','--check'],
 'browser':['node','tools/ui/art-memory-browser.cjs'],'binary':['cargo','build','--locked','--release','-p','spheres-web']}
env=dict(os.environ,CARGO_TARGET_DIR=str(base/'integration-target'),CARGO_BUILD_JOBS='4',SPHERES_ART_OUTPUT=str(base/'evidence'/('S22-prep-'+label+'-browser')))
expected=1 if lane=='budget-guard' else 0
proof={'revision':pin,'lane':lane,'command':commands[lane],'started_utc':now(),'clean_before':True,'runner_sha256':sha(__file__),'expected_exit_code':expected}
with prefix.with_suffix('.log').open('xb') as f:
 r=subprocess.run(commands[lane],cwd=repo,env=env,stdout=f,stderr=subprocess.STDOUT,**({'creationflags':subprocess.CREATE_NO_WINDOW} if os.name=='nt' else {}))
log=prefix.with_suffix('.log').read_text(encoding='utf8',errors='replace')
proof.update(exit_code=r.returncode,finished_utc=now(),clean_after=not git('status','--porcelain'),revision_after=git('rev-parse','HEAD'),log_sha256=sha(prefix.with_suffix('.log')))
proof['passed']=r.returncode==expected and proof['revision_after']==pin and proof['clean_after']
if lane=='budget-guard':
 over=len(json.loads((repo/'docs/art/P0_MEASUREMENTS.json').read_text())['over']);proof.update(budget_gate_passed=False,over_budget_configurations=over)
 proof['passed'] &= 'stale' not in log and log.count('over budget: ')==over and over>0
if lane=='focused':proof['counts']={k:int(v) for k,v in re.findall(r'# (tests|pass|fail|skipped) (\d+)',log)}
if lane=='binary':proof['binary_sha256']=sha(base/'integration-target/release/spheres-web.exe')
prefix.with_suffix('.json').write_text(json.dumps(proof,indent=2)+'\n',encoding='utf8')
print(json.dumps(proof,indent=2),flush=True)
if not proof['passed']:print(log[-4500:]);sys.exit(1)
