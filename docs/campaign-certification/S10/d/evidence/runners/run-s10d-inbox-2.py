import datetime, hashlib, json, os, pathlib, re, subprocess, sys
base=pathlib.Path(__file__).resolve().parent
repo=base/'integration'
pin=sys.argv[1]
label=sys.argv[2] if len(sys.argv)>2 else 'final'
assert re.fullmatch('[0-9a-f]{40}',pin) and re.fullmatch('[a-z0-9-]+',label)
def git(*args):return subprocess.check_output(['git','-c','core.longpaths=true',*args],cwd=repo,text=True).strip()
def sha(path):
    with open(path,'rb') as f:return hashlib.file_digest(f,'sha256').hexdigest()
assert not git('status','--porcelain')
driver=git('rev-parse','HEAD')
assert not git('diff','--name-only',pin,driver,'--','spheres-sim','spheres-cli','spheres-web','Cargo.toml','Cargo.lock')
web=json.loads((base/'evidence/S10-d-web-2.json').read_text())
build=json.loads((base/'evidence/S10-d-binary-2.json').read_text())
assert web['passed'] and build['passed'] and web['revision']==build['revision']==pin
test_binary=pathlib.Path(web['test_binary']['path'])
exe=pathlib.Path(build['binary']['path'])
assert sha(test_binary)==web['test_binary']['sha256'] and sha(exe)==build['binary']['sha256']
out=base/('evidence/S10d-inbox-'+label);out.mkdir(exist_ok=False)
fixture=base/'evidence/S10d-inbox-fixture-2'
env=dict(os.environ)
env.update(SPHERES_S10D_INBOX_FIXTURE_DIR=str(fixture),SPHERES_BINARY=str(exe),
    SPHERES_EXPECTED_REVISION=pin,SPHERES_S10D_INBOX_FIXTURE=str(fixture/'manifest.json'),
    SPHERES_INBOX_OUTPUT=str(out/'browser'),SPHERES_BROWSER_CHANNEL='chrome')
commands=[]
if not fixture.exists():
    commands.append(('export',[str(test_binary),'s10d_inbox_fixture_tests::s10d_export_disposable_diplomatic_inbox_fixture','--exact','--ignored','--nocapture','--test-threads=1']))
commands.append(('browser',['node','tools/ui/ci-diplomatic-inbox.cjs']))
proof={'revision':pin,'driver_revision':driver,'started_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),
    'runner_sha256':sha(__file__),'checks':[]}
for name,command in commands:
    log=out/(name+'.log')
    with log.open('xb') as stream:p=subprocess.run(command,cwd=repo,env=env,stdout=stream,stderr=subprocess.STDOUT,creationflags=subprocess.CREATE_NO_WINDOW)
    proof['checks'].append({'name':name,'command':command,'exit_code':p.returncode,'log':log.name,'log_sha256':sha(log)})
    if p.returncode:print(log.read_text(encoding='utf8',errors='replace')[-4000:],flush=True);break
proof['clean_after']=not git('status','--porcelain')
proof['driver_revision_after']=git('rev-parse','HEAD')
proof['binary_sha256_after']=sha(exe)
proof['passed']=all(c['exit_code']==0 for c in proof['checks']) and proof['checks'][-1]['name']=='browser' and proof['clean_after'] and proof['driver_revision_after']==driver and proof['binary_sha256_after']==build['binary']['sha256']
proof['finished_utc']=datetime.datetime.now(datetime.timezone.utc).isoformat()
(out/'result.json').write_text(json.dumps(proof,indent=2)+'\n',encoding='utf8')
print(json.dumps(proof,indent=2),flush=True)
sys.exit(0 if proof['passed'] else 1)
