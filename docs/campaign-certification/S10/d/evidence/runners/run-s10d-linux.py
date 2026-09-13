import os, sys, pathlib, json, hashlib, subprocess, datetime, re
base = pathlib.Path(__file__).resolve().parent
repo = base / 'integration'
pin = sys.argv[1]
assert re.fullmatch('[0-9a-f]{40}', pin)
out = base / 'evidence/S10d-linux-final'
out.mkdir(exist_ok=False)
env = dict(os.environ)
env.update(GIT_DIR='/mnt/c/Users/ridge/Spheres/.git/worktrees/integration',
    GIT_WORK_TREE=str(repo),GIT_OPTIONAL_LOCKS='0',CARGO_BUILD_JOBS='4',
    GIT_CONFIG_COUNT='1',GIT_CONFIG_KEY_0='core.autocrlf',GIT_CONFIG_VALUE_0='true')
git_exe = '/mnt/c/Program Files/Git/cmd/git.exe'
git_repo = 'C:/Users/ridge/Documents/Codex/2026-09-05/pick-up-the-spheres-game-on/work/campaign-certification/integration'
git_env = {k:v for k,v in env.items() if k not in ['GIT_DIR','GIT_WORK_TREE']}
git_env['GIT_OPTIONAL_LOCKS'] = '0'
def git(*args):
    return subprocess.check_output([git_exe,'-C',git_repo,'-c','core.longpaths=true',*args],env=git_env,text=True).strip()
def snapshot():
    value={'head':git('rev-parse','HEAD'),'status':git('status','--porcelain')}
    assert value == {'head':pin,'status':''},value
    return value
sha=lambda p:hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest()
stamp=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
record={'format':'spheres-s10d-linux/v1','revision':pin,'started_utc':stamp(),
    'scope':'Full native web suite and focused native agency/succession integration suites on Linux. No new Linux Node, external old-save or long-campaign qualification is claimed.',
    'runner_sha256':sha(__file__),'source_before':snapshot(),'checks':[]}
def persist(): (out/'result.json').write_text(json.dumps(record,indent=2)+'\n',encoding='utf8')
persist()
for name,command in [
    ('web',['cargo','test','--locked','--release','-p','spheres-web','--no-fail-fast']),
    ('agency',['cargo','test','--locked','--release','-p','spheres-sim','--test','agency','--test','s02_succession','--no-fail-fast'])
]:
    print('START '+name,flush=True)
    log=out/(name+'.log')
    with log.open('xb') as stream: p=subprocess.run(command,cwd=repo,env=env,stdout=stream,stderr=subprocess.STDOUT)
    text=log.read_text(encoding='utf8',errors='replace')
    totals=[tuple(map(int,m)) for m in re.findall(r'test result: (?:ok|FAILED)\. (\d+) passed; (\d+) failed; (\d+) ignored;',text)]
    record['checks'].append({'name':name,'command':command,'exit_code':p.returncode,'log':log.name,'log_sha256':sha(log),
        'totals':{'passed':sum(t[0] for t in totals),'failed':sum(t[1] for t in totals),'ignored':sum(t[2] for t in totals),'completed_targets':len(totals)},'source_after':snapshot()})
    persist();print('FINISH '+name+' exit='+str(p.returncode),flush=True)
    if p.returncode: print(text[-4000:],flush=True);break
record['passed']=len(record['checks'])==2 and all(c['exit_code']==0 and c['totals']['completed_targets']>0 and not c['totals']['failed'] for c in record['checks'])
record['finished_utc']=stamp();persist();print(json.dumps(record,indent=2),flush=True)
sys.exit(0 if record['passed'] else 1)
