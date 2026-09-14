"""Recheck Linux Node with its installed Python; retain unchanged native proofs."""
import copy,datetime,hashlib,json,os,pathlib,re,subprocess,sys,time
base=pathlib.Path(__file__).resolve().parent;repo=base/'integration'
pin=sys.argv[1];assert re.fullmatch('[a-f0-9]{40}',pin)
parent_file=base/'evidence/S11-final2-linux/result.json'
parent=json.loads(parent_file.read_text(encoding='utf8'));assert parent['revision']==pin
assert [c['name'] for c in parent['checks']]==['web','sim','integration','node']
assert all(c['passed'] for c in parent['checks'][:3]) and not parent['checks'][3]['passed']
out=base/'evidence/S11-final2-linux-recheck';out.mkdir(exist_ok=False)
def sha(p):
    with pathlib.Path(p).open('rb') as f:return hashlib.file_digest(f,'sha256').hexdigest()
def stamp():return datetime.datetime.now(datetime.timezone.utc).isoformat()
env=dict(os.environ);env.update(parent['environment']);env['SPHERES_AUDIT_PYTHON']='/usr/bin/python3'
assert pathlib.Path(env['SPHERES_AUDIT_PYTHON']).is_file()
git_env={k:v for k,v in env.items() if k not in ['GIT_DIR','GIT_WORK_TREE']}
def snapshot():
    args=['/mnt/c/Program Files/Git/cmd/git.exe','-C','C:/Users/ridge/Documents/Codex/2026-09-05/pick-up-the-spheres-game-on/work/campaign-certification/integration','-c','core.longpaths=true']
    def git(*more):return subprocess.check_output(args+list(more),env=git_env,text=True).strip()
    state={'head':git('rev-parse','HEAD'),'status':git('status','--porcelain')}
    assert state=={'head':pin,'status':''};return state
record=copy.deepcopy(parent)
record.update(status='running',passed=False,started_utc=stamp(),runner_sha256=sha(__file__),source_before=snapshot(),checks=[])
record.pop('finished_utc',None);record.pop('runner_sha256_after',None);record.pop('source_after',None)
record['environment']['SPHERES_AUDIT_PYTHON']=env['SPHERES_AUDIT_PYTHON']
record['native_parent']={'path':'../S11-final2-linux/result.json','sha256':sha(parent_file),'reused_checks':['web','sim','integration'],
    'reason':'Only Node interpreter setup changed; no source or native binary change. Native results are retained from the prior exact candidate, not claimed as rerun.'}
record['scope']='Full qualification composed from three unchanged successful native lanes and a fresh full Node run with the installed Python interpreter. The failed Node attempt is retained in native_parent.'
for check in parent['checks'][:3]:
    check=copy.deepcopy(check);assert sha(parent_file.parent/check['log'])==check['log_sha256']
    check['log']='../S11-final2-linux/'+check['log'];record['checks'].append(check)
def persist():(out/'result.json').write_text(json.dumps(record,indent=2)+'\n',encoding='utf8')
persist();print('START Linux Node recheck with /usr/bin/python3',flush=True)
started=time.monotonic();check={'name':'node','command':['node','tools/ui/run-unit.cjs'],'started_utc':stamp(),'source_before':snapshot(),'log':'node.log'}
with (out/'node.log').open('xb') as f:
    result=subprocess.run(check['command'],cwd=repo,env=env,stdout=f,stderr=subprocess.STDOUT)
log=(out/'node.log').read_text(encoding='utf8',errors='replace')
totals={name:int(re.findall(r'(?:ℹ|#)\s+'+word+r'\s+(\d+)',log)[-1]) for word,name in [('tests','tests'),('pass','passed'),('fail','failed'),('skipped','skipped')]}
check.update(finished_utc=stamp(),elapsed_seconds=time.monotonic()-started,exit_code=result.returncode,log_sha256=sha(out/'node.log'),totals=totals,source_after=snapshot())
check['passed']=result.returncode==0 and totals['passed']>0 and totals['failed']==0
record['checks'].append(check);record.update(source_after=snapshot(),runner_sha256_after=sha(__file__),finished_utc=stamp())
assert sha(parent_file)==record['native_parent']['sha256']
record['passed']=check['passed'] and record['runner_sha256']==record['runner_sha256_after'];record['status']='passed' if record['passed'] else 'failed'
persist();print(json.dumps({'passed':record['passed'],'result':str(out/'result.json'),'fresh_node':totals,'inherited_native_lanes':['web','sim','integration']}),flush=True)
sys.exit(0 if record['passed'] else 1)
