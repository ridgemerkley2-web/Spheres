import pathlib,subprocess,sys,os,json,hashlib,datetime,re
base=pathlib.Path(__file__).resolve().parent;repo=base/'s19-review-01';lane=sys.argv[1]
def git(*a):return subprocess.check_output(['git','-c','core.longpaths=true',*a],cwd=repo,text=True).strip()
def sha(p):
 with open(p,'rb') as f:return hashlib.file_digest(f,'sha256').hexdigest()
pin=git('rev-parse','HEAD');assert not git('status','--porcelain')
out=base/'evidence/S19-review-01';out.mkdir(exist_ok=True);prefix=out/lane;assert not prefix.with_suffix('.log').exists()
commands={'focused':['node','--test','--test-reporter=tap',*['tools/ui/check_'+x+'.cjs' for x in ['tutorial_model','advisor_model','guidance_outcomes','guidance_ui','guidance_host']]],
 'native':['cargo','test','--locked','--release','-p','spheres-web','--bin','spheres-web','guidance'],
 'binary':['cargo','build','--locked','--release','-p','spheres-web']}
env=dict(os.environ,CARGO_TARGET_DIR=str(base/'s19-review-target'),CARGO_BUILD_JOBS='4')
proof={'source_revision':pin,'lane':lane,'command':commands[lane],'started_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'clean_before':True,'runner_sha256':sha(__file__)}
with prefix.with_suffix('.log').open('xb') as f:r=subprocess.run(commands[lane],cwd=repo,env=env,stdout=f,stderr=subprocess.STDOUT,creationflags=subprocess.CREATE_NO_WINDOW)
log=prefix.with_suffix('.log').read_text(encoding='utf8',errors='replace')
proof.update(exit_code=r.returncode,finished_utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),clean_after=not git('status','--porcelain'),revision_after=git('rev-parse','HEAD'),log_sha256=sha(prefix.with_suffix('.log')))
proof['counts']={k:int(v) for k,v in re.findall(r'# (tests|pass|fail|skipped) (\d+)',log)} if lane=='focused' else re.findall(r'test result: (?:ok|FAILED)\. (\d+) passed; (\d+) failed; (\d+) ignored;',log)
if lane=='binary':proof['binary_sha256']=sha(base/'s19-review-target/release/spheres-web.exe')
proof['passed']=r.returncode==0 and proof['clean_after'] and proof['revision_after']==pin
prefix.with_suffix('.json').write_text(json.dumps(proof,indent=2)+'\n',encoding='utf8')
print(json.dumps(proof,indent=2),flush=True)
if not proof['passed']:print(log[-5000:]);sys.exit(1)
