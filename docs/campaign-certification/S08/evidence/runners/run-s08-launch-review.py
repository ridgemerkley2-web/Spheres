"""Launch a new isolated review from qualified S08 evidence, never an original save."""
import datetime,hashlib,json,pathlib,re,shutil,socket,subprocess,sys,time,urllib.request
base=pathlib.Path(__file__).resolve().parent
pin=sys.argv[1]
buildfile,exportfile,browserfile=map(lambda p:pathlib.Path(p).resolve(),sys.argv[2:5])
assert re.fullmatch('[0-9a-f]{40}',pin)
def read(p):return json.loads(p.read_text(encoding='utf-8-sig'))
def sha(p):
    with p.open('rb') as f:return hashlib.file_digest(f,'sha256').hexdigest()
buildproof,export,browser=map(read,[buildfile,exportfile,browserfile])
assert buildproof['passed'] and buildproof['revision_before']==pin==buildproof['revision_after']
assert export['passed'] and export['candidate']==pin and browser['passed'] and browser['build']['revision']==pin
source=pathlib.Path(buildproof['binary']).resolve()
assert sha(source)==buildproof['binary_sha256']
exportdir=pathlib.Path(export['export_directory'])
inputs=[]
for suffix,slot in [('fresh-before-decisions','s08-fresh-tonga'),('ready-before-purchase','s08-tonga-market-ready'),('purchased','s08-tonga-paid-transit'),('delivered','s08-tonga-delivered')]:
    matches=list(exportdir.glob('*-'+suffix+'.campaign.json'));assert len(matches)==1
    original=matches[0]
    record=next(r for r in export['archive_phases'] if r['file']==original.name)
    assert sha(original)==record['sha256']
    inputs.append((original,slot))
completed=pathlib.Path(browser['run'])/'saves/s08-import-complete.json'
assert completed.exists() and browser.get('maintenance',{}),'Review requires the browser-proven paid upkeep step'
inputs.append((completed,'s08-tonga-in-service'))
def free_port():
    for candidate in range(7846,7861):
        with socket.socket() as probe:
            try:probe.bind(('127.0.0.1',candidate));return candidate
            except OSError:continue
    return None
port=free_port()
assert port is not None
with socket.socket() as probe:probe.bind(('127.0.0.1',port))
run=base/('review-S08-'+datetime.datetime.now().strftime('%Y%m%d-%H%M%S'))
run.mkdir();(run/'saves').mkdir();exe=run/'spheres-web.exe';shutil.copy2(source,exe)
copies=[]
for original,slot in inputs:
    target=run/'saves'/(slot+'.json');shutil.copy2(original,target)
    assert sha(original)==sha(target)
    copies.append({'source':str(original),'copy':str(target),'sha256':sha(target)})
with (run/'server.stdout.log').open('xb') as out,(run/'server.stderr.log').open('xb') as err:
    proc=subprocess.Popen([str(exe),'--port',str(port),'--no-open'],cwd=run,stdout=out,stderr=err,creationflags=subprocess.CREATE_NO_WINDOW)
url=f'http://127.0.0.1:{port}'
def request(route,data=None):
    req=urllib.request.Request(url+route,data=None if data is None else json.dumps(data).encode(),headers={'Content-Type':'application/json','Connection':'close'})
    with urllib.request.urlopen(req,timeout=60) as r:return json.load(r)
try:
    for attempt in range(100):
        assert proc.poll() is None
        try:build=request('/api/build');break
        except (OSError,TimeoutError):time.sleep(.1)
    else:raise RuntimeError('Review startup timeout')
    assert build['revision']==pin[:12]
    state=request('/api/load',{'slot':'s08-tonga-in-service'})
    assert state['player']=='Tonga'
    assert all(sha(pathlib.Path(c['source']))==c['sha256']==sha(pathlib.Path(c['copy'])) for c in copies)
    record={'url':url,'pid':proc.pid,'directory':str(run),'runtime_revision':pin,'executable_sha256':sha(exe),
        'date':state['date'],'player':state['player'],'copies':copies,'build':build,
        'qualification':[{'path':str(p),'sha256':sha(p)} for p in [buildfile,exportfile,browserfile]],
        'scope':'Isolated review copy with earned supplier stock, paid imports and browser-proven maintenance; earlier campaigns and servers retained.'}
    for target in [run/'review-launch.json',base/'S08-review-launch.json']:
        with target.open('x',encoding='utf-8') as f:json.dump(record,f,indent=2);f.write('\n')
    print(json.dumps(record,indent=2),flush=True)
except BaseException:
    proc.terminate();proc.wait(timeout=10);raise
