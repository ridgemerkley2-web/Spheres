import datetime, hashlib, json, os, subprocess, sys
from pathlib import Path
ROOT=Path('D:/spheres-offload/codex-next-20260928/review-c01-43-20261001')
OUT=Path(__file__).parent/'validation'
OUT.mkdir(exist_ok=True)
label=sys.argv[1]
cmd=sys.argv[2:]
log=OUT/(label+'.log')
record=OUT/(label+'.json')
assert not log.exists() and not record.exists()
start=datetime.datetime.now(datetime.timezone.utc).isoformat()
env=dict(os.environ,PYTHONPATH=str(ROOT/'tools/avatars'))
r=subprocess.run(cmd,cwd=ROOT,env=env,stdout=subprocess.PIPE,stderr=subprocess.STDOUT)
log.write_bytes(r.stdout)
record.write_text(json.dumps({'command':cmd,'cwd':str(ROOT),'started_utc':start,'finished_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'exit_code':r.returncode,'output':{'path':str(log),'bytes':len(r.stdout),'sha256':hashlib.sha256(r.stdout).hexdigest()}},indent=2)+'\n',encoding='utf-8',newline='\n')
print(r.stdout.decode('utf-8',errors='replace'))
print('EXIT_CODE',r.returncode)
