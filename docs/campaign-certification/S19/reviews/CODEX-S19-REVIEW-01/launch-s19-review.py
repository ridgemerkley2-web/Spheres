import pathlib, subprocess, hashlib, json, datetime, shutil, socket, time, urllib.request
base = pathlib.Path(__file__).resolve().parent
def read(p): return json.loads(p.read_text(encoding='utf-8'))
def sha(p):
    with p.open('rb') as f: return hashlib.file_digest(f, 'sha256').hexdigest()
build = read(base/'evidence/S19-review-01/binary.json')
browser = read(base/'evidence/S19-review-01/candidate-navigation/result.json')
pin = build['source_revision']
assert build['passed'] and browser['passed'] and browser['revision'] == pin
source = base/'s19-review-target/release/spheres-web.exe'
assert sha(source) == build['binary_sha256'] == browser['binary_sha256']
saved = base/'evidence/S19-review-01/candidate-navigation/server/saves/s19-route-review.json'
run = base/('review-S19-candidate-'+datetime.datetime.now().strftime('%Y%m%d-%H%M%S'))
run.mkdir(); (run/'saves').mkdir()
exe = run/'spheres-web.exe'; target = run/'saves/s19-preview.json'
shutil.copy2(source, exe); shutil.copy2(saved, target)
assert sha(saved) == sha(target)
for port in range(7863, 7890):
    with socket.socket() as probe:
        try: probe.bind(('127.0.0.1', port)); break
        except OSError: continue
else: raise RuntimeError('No free preview port')
with (run/'stdout.log').open('xb') as out, (run/'stderr.log').open('xb') as err:
    p = subprocess.Popen([str(exe), '--port', str(port), '--no-open'], cwd=run, stdout=out, stderr=err, creationflags=subprocess.CREATE_NO_WINDOW)
url = f'http://127.0.0.1:{port}'
def req(route, data=None):
    q = urllib.request.Request(url+route, data=None if data is None else json.dumps(data).encode(), headers={'Content-Type':'application/json'})
    with urllib.request.urlopen(q, timeout=30) as r: return json.load(r)
try:
    until = time.monotonic()+30
    while True:
        assert p.poll() is None
        try: b = req('/api/build'); break
        except OSError:
            if time.monotonic()>until: raise
            time.sleep(.1)
    assert b['revision'] == pin[:12]
    assets = {}
    for name in ['tutorial-model.js','advisor-model.js','guidance-ui.js','guidance-ui.css','equipment-model.js']:
        with urllib.request.urlopen(url+'/'+name,timeout=30) as r: asset=r.read()
        assert asset == (base/'s19-review-01/spheres-web/ui'/name).read_bytes()
        assets[name] = hashlib.sha256(asset).hexdigest()
    s = req('/api/load', {'slot':'s19-preview'})
    assert s['player'] == 'France' and s['date'] == '3 Jan 1990'
    info = {'url':url, 'pid':p.pid, 'directory':str(run), 'runtime_revision':pin, 'executable_sha256':sha(exe), 'player':s['player'], 'date':s['date'], 'build':b,
        'served_assets_sha256':assets, 'save_copy':{'source':str(saved),'path':str(target),'sha256':sha(target)},
        'scope':'Isolated S19 in-progress candidate, not integrated or certified. Actual disposable construction campaign copied from browser review. Launcher load is not a browser save/resume receipt. Main playset and prior servers remain unchanged.'}
    with (base/'S19-review-launch.json').open('x',encoding='utf8') as f: json.dump(info,f,indent=2)
    print(json.dumps(info,indent=2))
except:
    p.terminate(); raise
