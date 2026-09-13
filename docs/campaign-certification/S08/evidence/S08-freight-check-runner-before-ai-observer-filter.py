import datetime,hashlib,json,os,pathlib,re,subprocess,sys
base=pathlib.Path(__file__).resolve().parent;repo=base/'integration'
label=sys.argv[1];assert re.fullmatch('[A-Za-z0-9_-]+',label)
out=base/'evidence'/('S08-freight-checks-'+label);out.mkdir(exist_ok=False)
def sha(p):
    with pathlib.Path(p).open('rb') as f:return hashlib.file_digest(f,'sha256').hexdigest()
def git(*a):return subprocess.check_output(['git','-c','core.longpaths=true',*a],cwd=repo,stderr=subprocess.PIPE)
def source():
    files=git('diff','--name-only','HEAD').decode().splitlines()+git('ls-files','--others','--exclude-standard').decode().splitlines()
    return {'head':git('rev-parse','HEAD').decode().strip(),'status':git('status','--porcelain').decode(),'files':{p:sha(repo/p) for p in sorted(set(files))},'diff_sha256':hashlib.sha256(git('diff','HEAD')).hexdigest()}
record={'scope':'Developmental focused semantic checks; not final candidate qualification.','before':source(),'runner_sha256':sha(__file__),'checks':[]}
(out/'source.patch').write_bytes(git('diff','HEAD'))
env=dict(os.environ,CARGO_TARGET_DIR=str(base/'integration-target'))
commands=[('root-sim',['cargo','test','--locked','--release','-p','spheres-sim','--lib','s08_','--','--nocapture','--test-threads=1']),('route-context',['cargo','test','--locked','--release','-p','spheres-sim','--lib','contract_forecast_context','--','--nocapture','--test-threads=1']),('market-cargo',['cargo','test','--locked','--release','-p','spheres-sim','--lib','spot_opening_pending','--','--nocapture','--test-threads=1']),('new-inbound',['cargo','test','--locked','--release','-p','spheres-sim','--lib','new_inbound_commodity_short_circuit','--','--nocapture','--test-threads=1']),('route-access',['cargo','test','--locked','--release','-p','spheres-sim','--lib','freight_access_read','--','--nocapture','--test-threads=1']),('clearing-access',['cargo','test','--locked','--release','-p','spheres-sim','--lib','clearing_access_reuse','--','--nocapture','--test-threads=1']),('commerce',['cargo','test','--locked','--release','-p','spheres-sim','--lib','commerce_','--','--nocapture','--test-threads=1']),('web-reads',['cargo','test','--locked','--release','-p','spheres-web','s08_','--','--nocapture','--test-threads=1'])]
for name,command in commands:
    started=datetime.datetime.now(datetime.timezone.utc).isoformat()
    with (out/(name+'.log')).open('xb') as log:r=subprocess.run(command,cwd=repo,env=env,stdout=log,stderr=subprocess.STDOUT,creationflags=subprocess.CREATE_NO_WINDOW)
    content=(out/(name+'.log')).read_text(errors='replace');totals=re.findall(r'test result: (?:ok|FAILED). (\d+) passed; (\d+) failed; (\d+) ignored;',content)
    check={'name':name,'command':command,'started_utc':started,'exit_code':r.returncode,'totals':totals,'log_sha256':sha(out/(name+'.log'))}
    check['passed']=r.returncode==0 and bool(totals) and sum(int(t[0]) for t in totals)>0 and all(int(t[1])==0 for t in totals)
    record['checks'].append(check);(out/'result.json').write_text(json.dumps(record,indent=2)+'\n');print(json.dumps(check),flush=True)
    if not check['passed']:break
record['after']=source();record['passed']=len(record['checks'])==len(commands) and all(c['passed'] for c in record['checks']) and record['before']==record['after']
(out/'result.json').write_text(json.dumps(record,indent=2)+'\n');sys.exit(0 if record['passed'] else 1)
