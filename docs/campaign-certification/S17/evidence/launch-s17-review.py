"""Launch a separately copied, qualified S17 review campaign."""
import datetime, hashlib, json, pathlib, shutil, socket, subprocess, sys, time, urllib.request
base=pathlib.Path(__file__).resolve().parent
def read(p): return json.loads(pathlib.Path(p).read_text(encoding='utf-8-sig'))
def sha(p):
    with open(p,'rb') as f: return hashlib.file_digest(f,'sha256').hexdigest()
pin=sys.argv[1]
buildfile=base/'evidence/S17-final6-binary.json'
browserproof=read(base/'evidence/S17-final6-browser.json')
proof=read(buildfile)
matches=list((base/'evidence/S17-browser-final6-browser').glob('staff-*/result.json'))
assert len(matches)==1
browserfile=matches[0]; browser=read(browserfile)
assert proof['passed'] and browserproof['passed'] and browser['passed']
assert proof['revision']==browser['revision']==browserproof['revision']==pin
source=pathlib.Path(proof['binary']['path'])
assert sha(source)==proof['binary']['sha256']==browser['binary_sha256']
saved=base/'evidence/S17-fixture-final6/active.json'
refs=[x for x in browser['world_comparisons'] if x['message']=='active']
assert len(refs)==1 and refs[0]['ignored_paths']==[]
run=base/('review-S17-'+datetime.datetime.now().strftime('%Y%m%d-%H%M%S'))
run.mkdir(); (run/'saves').mkdir()
exe=run/'spheres-web.exe'; target=run/'saves/s17-military-staff.json'
shutil.copy2(source,exe); shutil.copy2(saved,target)
assert sha(source)==sha(exe) and sha(saved)==sha(target)
verify=run/'native-verification'; verify.mkdir()
worker=base/'integration/tools/ui/archive-worker.py'
result=subprocess.run([sys.executable,str(worker),str(target),str(verify/'copied-save.canonical'),'France'],capture_output=True,text=True,encoding='utf8',timeout=120,creationflags=subprocess.CREATE_NO_WINDOW)
assert result.returncode==0,result.stderr
audited=json.loads(result.stdout)
assert audited['canonical']['ignored_paths']==[] and audited['canonical']['sha256']==refs[0]['sha256']
(verify/'copied-save.json').write_text(json.dumps(audited,indent=2)+'\n',encoding='utf8')
port=None
for candidate in range(7859,7875):
    with socket.socket() as probe:
        try: probe.bind(('127.0.0.1',candidate)); port=candidate; break
        except OSError: continue
assert port
with (run/'server.stdout.log').open('xb') as out, (run/'server.stderr.log').open('xb') as err:
    process=subprocess.Popen([str(exe),'--port',str(port),'--no-open'],cwd=run,stdout=out,stderr=err,creationflags=subprocess.CREATE_NO_WINDOW)
url=f'http://127.0.0.1:{port}'
def request(route,data=None):
    req=urllib.request.Request(url+route,data=None if data is None else json.dumps(data).encode(),headers={'Content-Type':'application/json','Connection':'close'})
    with urllib.request.urlopen(req,timeout=30) as response: return json.load(response)
try:
    until=time.monotonic()+30
    while True:
        assert process.poll() is None
        try: build=request('/api/build'); break
        except OSError:
            if time.monotonic()>until: raise
            time.sleep(.1)
    assert build['revision']==pin[:12]
    state=request('/api/load',{'slot':'s17-military-staff'})
    assert state['player']=='France'
    equipment=request('/api/equipment?session_id='+state['session_id'])
    assert equipment['flight']['staff']['status']=='Active'
    record={'url':url,'pid':process.pid,'directory':str(run),'runtime_revision':pin,'executable_sha256':sha(exe),'player':state['player'],'date':state['date'],'build':build,
        'save_copy':{'source':str(saved),'copy':str(target),'sha256':sha(target)},
        'native_verification':{'canonical_sha256':audited['canonical']['sha256'],'bytes':audited['canonical']['bytes'],'ignored_paths':[], 'matches_native_and_browser_active_checkpoint':True,'worker_sha256':sha(worker)},
        'evidence':[{'path':str(p),'sha256':sha(p)} for p in [buildfile,browserfile]],
        'staff':equipment['flight']['staff'],
        'scope':'Separate authored France S16 starting forces with S17 military staff active after four ordinary days. Autonomous Italian missions consume actual stores. No historical or unassisted campaign claim. Original review servers and campaigns preserved.'}
    with (base/'S17-review-launch.json').open('x',encoding='utf8') as f: json.dump(record,f,indent=2); f.write('\n')
    print(json.dumps({k:record[k] for k in ['url','pid','directory','runtime_revision','player','date']},indent=2),flush=True)
except:
    process.terminate(); raise



