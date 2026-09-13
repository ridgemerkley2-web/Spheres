import datetime, hashlib, json, pathlib, subprocess, sys
base=pathlib.Path(__file__).resolve().parent; repo=base/'integration'; pin=sys.argv[1]
out=base/'evidence/S10b-intake-final-1'; out.mkdir(exist_ok=False)
git=lambda *a:subprocess.check_output(['git','-c','core.longpaths=true',*a],cwd=repo,text=True).strip()
sha=lambda p:hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest()
assert git('rev-parse','HEAD')==pin and not git('status','--porcelain')
proof={'revision':pin,'runner_sha256':sha(__file__),'started_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'checks':[]}
commands=[['python','-X','utf8','-m','unittest','discover','-s','tools/avatars','-p','test_campaign*.py']]
commands += [['python','-X','utf8','tools/avatars/'+name+'.py','--check'] for name in ['campaign_census','import_cnccfp_census','campaign_research']]
commands += [['node','--check','tools/ui/ci-government-country-matrix.cjs']]
for i,cmd in enumerate(commands):
    log=out/f'check-{i+1}.log'
    with log.open('xb') as stream: result=subprocess.run(cmd,cwd=repo,stdout=stream,stderr=subprocess.STDOUT)
    proof['checks'].append({'command':cmd,'exit_code':result.returncode,'log':log.name,'log_sha256':sha(log)})
proof['finished_utc']=datetime.datetime.now(datetime.timezone.utc).isoformat()
proof['clean_after']=not git('status','--porcelain') and git('rev-parse','HEAD')==pin
proof['passed']=all(c['exit_code']==0 for c in proof['checks']) and proof['clean_after']
(out/'result.json').write_text(json.dumps(proof,indent=2)+'\n',encoding='utf-8')
print(json.dumps(proof,indent=2));sys.exit(0 if proof['passed'] else 1)
