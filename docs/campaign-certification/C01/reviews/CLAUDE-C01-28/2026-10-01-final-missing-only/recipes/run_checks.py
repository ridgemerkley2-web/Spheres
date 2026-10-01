from pathlib import Path
import datetime, hashlib, json, subprocess, sys, time
root=Path(__file__).resolve().parent
cwd=Path('D:/spheres-offload/codex-next-20260928/review-c01-28-20261001')
revision=subprocess.check_output(['git','rev-parse','HEAD'],cwd=cwd,text=True).strip()
assert revision=='6133f79b5b00126d4733453fe42839c1ca7390c4'
command=[sys.executable,'-B','-X','utf8','-m','unittest','discover','-s','tools/avatars','-p','test_russia*.py','-v']
started=datetime.datetime.now(datetime.timezone.utc).isoformat();t=time.monotonic()
result=subprocess.run(command,cwd=cwd,stdout=subprocess.PIPE,stderr=subprocess.STDOUT)
(root/'russia-tests.log').write_bytes(result.stdout)
record={'cwd':str(cwd),'revision':revision,'command':command,'started_utc':started,'elapsed_seconds':round(time.monotonic()-t,3),'exit_code':result.returncode,'log':'russia-tests.log','log_bytes':len(result.stdout),'log_sha256':hashlib.sha256(result.stdout).hexdigest(),'scope':'Unchanged isolated reviewed C01-28 packet; no new data edits; not the current combined integration test suite.'}
(root/'validation.json').write_text(json.dumps(record,indent=2)+'\n',encoding='utf-8',newline='\n')
print(json.dumps(record));print(result.stdout.decode('utf-8','replace')[-3000:]);sys.exit(result.returncode)
