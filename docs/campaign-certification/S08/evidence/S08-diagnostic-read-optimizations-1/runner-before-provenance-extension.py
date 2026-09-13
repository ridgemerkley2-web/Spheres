"""Retained developmental builds/diagnoses; never campaign acceptance evidence."""
import datetime,hashlib,json,os,pathlib,re,subprocess,sys
base=pathlib.Path(__file__).resolve().parent;repo=base/'integration'
lane,label=sys.argv[1:3];assert lane in ('build','read','daily') and re.fullmatch('[A-Za-z0-9_-]+',label)
out=base/'evidence'/('S08-diagnostic-'+label);out.mkdir(exist_ok=False)
def sha(p):
    with pathlib.Path(p).open('rb') as f:return hashlib.file_digest(f,'sha256').hexdigest()
def git(*args):return subprocess.check_output(['git','-c','core.longpaths=true',*args],cwd=repo)
def source():
    names=['spheres-sim/src/lib.rs','spheres-web/src/performance.rs','spheres-web/src/main.rs','spheres-web/src/equipment_view.rs','spheres-web/src/equipment_ammunition_view.rs','spheres-web/src/company_view.rs','spheres-web/src/company_ammunition_view.rs','spheres-web/src/s08_performance_diagnostics.rs']
    names=sorted(set(names+git('diff','--name-only','HEAD').decode().splitlines()))
    return {'head':git('rev-parse','HEAD').decode().strip(),'status':git('status','--porcelain').decode(),'files':{n:sha(repo/n) for n in names if (repo/n).is_file()},'diff_sha256':hashlib.sha256(git('diff','HEAD')).hexdigest()}
before=source();(out/'source.patch').write_bytes(git('diff','HEAD'))
for n in before['files']:
    p=out/'source'/n;p.parent.mkdir(parents=True,exist_ok=True);p.write_bytes((repo/n).read_bytes())
env=os.environ.copy();env['CARGO_TARGET_DIR']=str(base/'integration-target')
if lane=='build':command=['cargo','test','--locked','--release','-p','spheres-web','--no-run']
else:
    binary=pathlib.Path(sys.argv[3]).resolve();command=[str(binary),'performance::'+('s08_read_model_diagnosis' if lane=='read' else 's08_daily_subsystem_diagnosis'),'--ignored','--exact','--nocapture','--test-threads=1']
    env['SPHERES_S08_DIAGNOSTIC_INPUT']=str((base/'evidence/S08-genuine-supplier-final-1/export/2127-purchased.campaign.json').resolve())
    env['SPHERES_S08_DIAGNOSTIC_OUT']=str(out/'diagnosis.json')
record={'scope':'Developmental diagnostic only; no qualification or threshold decision.','lane':lane,'source_before':before,'command':command,'started_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'runner_sha256':sha(__file__)}
with (out/'stdout-stderr.log').open('xb') as log:
    r=subprocess.run(command,cwd=repo,env=env,stdout=log,stderr=subprocess.STDOUT,creationflags=subprocess.CREATE_NO_WINDOW)
record.update(exit_code=r.returncode,source_after=source(),finished_utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),log_sha256=sha(out/'stdout-stderr.log'))
record['source_unchanged']=record['source_after']==before
if lane!='build':record['binary']={'path':str(binary),'sha256':sha(binary)}
record['passed']=r.returncode==0 and record['source_unchanged']
(out/'result.json').write_text(json.dumps(record,indent=2)+'\n')
print(json.dumps({'passed':record['passed'],'evidence':str(out),'exit_code':r.returncode}),flush=True)
sys.exit(0 if record['passed'] else 1)
