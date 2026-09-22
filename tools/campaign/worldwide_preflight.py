"""Disposable native startup/recovery preflight; not S24 browser or CP1 qualification.

Example: python tools/campaign/worldwide_preflight.py --binary <release-exe>
  --out <new-directory> --expected-revision <40-char-commit> [--limit 3]
Only launches its own server. Existing servers and save directories are never used.
"""
import argparse, datetime, hashlib, json, math, pathlib, socket, subprocess, time, urllib.parse, urllib.request


def canonical(value):
    return json.dumps(value, sort_keys=True, separators=(',', ':'), ensure_ascii=False, allow_nan=False).encode()


def digest(value):
    return hashlib.sha256(canonical(value)).hexdigest()


def archive_projection(path):
    value = json.loads(path.read_text(encoding='utf-8'))
    assert value.get('format') == 'spheres-campaign' and value.get('version') == 1
    # Only wall-clock file metadata is excluded; world, history, log and journey remain.
    value.pop('saved_unix', None)
    return value


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--binary', required=True, type=pathlib.Path)
    parser.add_argument('--out', required=True, type=pathlib.Path)
    parser.add_argument('--expected-revision', required=True)
    parser.add_argument('--limit', type=int)
    parser.add_argument('--seed', type=int, default=17)
    args = parser.parse_args()
    repo = pathlib.Path(__file__).resolve().parents[2]
    revision = subprocess.check_output(['git','rev-parse','HEAD'],cwd=repo,text=True).strip()
    assert revision == args.expected_revision and len(revision) == 40
    assert not subprocess.check_output(['git','status','--porcelain'],cwd=repo,text=True).strip(), 'Use a clean source revision'
    assert args.limit is None or args.limit > 0
    binary=args.binary.resolve(strict=True);out=args.out.resolve();out.mkdir(parents=True,exist_ok=False)
    run=out/'server';run.mkdir()
    with socket.socket() as probe:
        probe.bind(('127.0.0.1',0));port=probe.getsockname()[1]
    url=f'http://127.0.0.1:{port}'
    proof={'source_revision':revision,'binary_sha256':hashlib.sha256(binary.read_bytes()).hexdigest(),
        'scope':'Native API preflight of ordinary 1990 starters: reads, seven days and full campaign save/load. No browser interaction, successor activation, long campaign, adversarial recovery or content certification.',
        'started_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'seed':args.seed,'cases':[], 'passed':False}
    def write(): (out/'result.json').write_text(json.dumps(proof,indent=2,ensure_ascii=False)+'\n',encoding='utf8')
    def req(route,payload=None):
        q=urllib.request.Request(url+route,data=None if payload is None else canonical(payload),headers={'Content-Type':'application/json'})
        with urllib.request.urlopen(q,timeout=60) as response: return json.load(response)
    flags=getattr(subprocess,'CREATE_NO_WINDOW',0)
    with (out/'server.log').open('xb') as log:
        process=subprocess.Popen([str(binary),'--port',str(port),'--no-open'],cwd=run,stdout=log,stderr=subprocess.STDOUT,creationflags=flags)
        try:
            until=time.monotonic()+30
            while True:
                assert process.poll() is None,'Disposable server exited during boot'
                try: build=req('/api/build');break
                except OSError:
                    if time.monotonic()>until:raise
                    time.sleep(.1)
            assert build['revision']==revision[:12]
            assert pathlib.Path(build['save_directory']).resolve()==run
            proof['build']=build
            initial=req('/api/new',{'seed':args.seed})
            nations=sorted(n['id'] for n in initial['nations'] if n['alive'])
            assert len(nations)==len(set(nations))==137,'Expected the documented 137 starting nations'
            proof['roster']=nations;proof['coverage']='all_starters' if args.limit is None else 'limited_pilot'
            for nation in nations[:args.limit]:
                started=time.monotonic();case={'nation':nation,'passed':False,'checks':[]}
                try:
                    state=req('/api/new',{'seed':args.seed,'nation':nation})
                    assert state['player']==nation and [state['year'],state['month'],state['day']]==[1990,1,1]
                    assert isinstance(state['districts'],dict)
                    me=next(n for n in state['nations'] if n['id']==nation);assert me['alive']
                    case['checks'].append('start_and_map_ownership_state')
                    for route in ['/api/government','/api/cash-flow','/api/guidance']:
                        reading=req(route+'?session_id='+urllib.parse.quote(state['session_id']))
                        if route=='/api/guidance':assert reading['state']['player']==nation and reading['state']['session_id']==state['session_id']
                        else:assert reading['nation']==nation
                        case['checks'].append(route[5:])
                    assert digest(req('/api/state'))==digest(state),'Read-only views changed campaign state'
                    payload={'days':7,'commands':[],'session_id':state['session_id'],'player_context':nation,'client_id':'worldwide-preflight','request_seq':1}
                    advanced=req('/api/advance',payload)
                    assert advanced['player']==nation and [advanced['year'],advanced['month'],advanced['day']]==[1990,1,8]
                    # Retrying the same protected turn must not advance another week.
                    assert digest(req('/api/advance',payload))==digest(advanced),'Duplicate turn changed campaign'
                    case['checks'].extend(['seven_days','duplicate_turn'])
                    # Reuse only these scratch slots inside our newly-created server directory.
                    # At most two current files plus native backups are retained, not 137 full worlds.
                    req('/api/save',{'slot':'preflight-before','session_id':advanced['session_id']})
                    before=archive_projection(run/'saves/preflight-before.json')
                    loaded=req('/api/load',{'slot':'preflight-before'})
                    assert loaded['session_id']!=advanced['session_id'] and loaded['player']==nation
                    assert [loaded['year'],loaded['month'],loaded['day']]==[1990,1,8]
                    req('/api/save',{'slot':'preflight-after','session_id':loaded['session_id']})
                    after=archive_projection(run/'saves/preflight-after.json')
                    before_hash=digest(before);after_hash=digest(after)
                    assert before_hash==after_hash,'Campaign archive changed across load/save (world, history, log, journey included)'
                    case.update(passed=True,date=loaded['date'],archive_sha256=before_hash,roundtrip_sha256=after_hash)
                    case['checks'].append('full_archive_roundtrip')
                except Exception as error:case['error']=str(error)
                case['elapsed_seconds']=round(time.monotonic()-started,3);proof['cases'].append(case);write()
                print(f"{len(proof['cases'])}/{len(nations[:args.limit])} {nation}: {'PASS' if case['passed'] else 'FAIL '+case['error']}",flush=True)
                if process.poll() is not None:break
            proof['passed']=len(proof['cases'])==len(nations[:args.limit]) and all(c['passed'] for c in proof['cases'])
            proof['all_137_passed']=proof['passed'] and args.limit is None
        finally:
            process.terminate()
            try:process.wait(timeout=10)
            except subprocess.TimeoutExpired:process.kill();process.wait(timeout=10)
            proof['finished_utc']=datetime.datetime.now(datetime.timezone.utc).isoformat()
            proof['source_clean_after']=not subprocess.check_output(['git','status','--porcelain'],cwd=repo,text=True).strip()
            proof['source_revision_after']=subprocess.check_output(['git','rev-parse','HEAD'],cwd=repo,text=True).strip()
            proof['passed']=proof['passed'] and proof['source_clean_after'] and proof['source_revision_after']==revision
            write()
    if not proof['passed']:raise SystemExit(1)


if __name__=='__main__':main()
