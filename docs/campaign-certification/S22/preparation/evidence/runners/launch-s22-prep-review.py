import pathlib,subprocess,hashlib,json,datetime,shutil,socket,time,urllib.request,sys
base=pathlib.Path(__file__).resolve().parent;pin=sys.argv[1]
def read(p):return json.loads(pathlib.Path(p).read_text(encoding='utf-8-sig'))
def sha(p):
 with open(p,'rb') as f:return hashlib.file_digest(f,'sha256').hexdigest()
build=read(base/'evidence/S22-prep-final1-binary.json');browser=read(base/'evidence/S22-prep-final1-browser.json')
assert build['passed'] and browser['passed'] and build['revision']==browser['revision']==pin
source=base/'integration-target/release/spheres-web.exe';assert sha(source)==build['binary_sha256']
old=read(base/'S21-review-launch.json');saved=pathlib.Path(old['save_copy']['path'])
run=base/('review-S22-prep-'+datetime.datetime.now().strftime('%Y%m%d-%H%M%S'));run.mkdir();(run/'saves').mkdir()
exe=run/'spheres-web.exe';target=run/'saves/s22-art-review.json';shutil.copy2(source,exe);shutil.copy2(saved,target)
assert sha(saved)==sha(target)
for port in range(7862,7890):
 with socket.socket() as probe:
  try:probe.bind(('127.0.0.1',port));break
  except OSError:continue
else:raise RuntimeError('No free review port')
with (run/'stdout.log').open('xb') as out,(run/'stderr.log').open('xb') as err:
 p=subprocess.Popen([str(exe),'--port',str(port),'--no-open'],cwd=run,stdout=out,stderr=err,creationflags=subprocess.CREATE_NO_WINDOW)
url=f'http://127.0.0.1:{port}'
def req(route,data=None):
 q=urllib.request.Request(url+route,data=None if data is None else json.dumps(data).encode(),headers={'Content-Type':'application/json'})
 with urllib.request.urlopen(q,timeout=30) as r:return json.load(r)
try:
 until=time.monotonic()+30
 while True:
  assert p.poll() is None
  try:b=req('/api/build');break
  except OSError:
   if time.monotonic()>until:raise
   time.sleep(.1)
 assert b['revision']==pin[:12]
 with urllib.request.urlopen(url+'/equipment-model.js',timeout=30) as r:asset=r.read()
 assert asset==(base/'integration/spheres-web/ui/equipment-model.js').read_bytes()
 s=req('/api/load',{'slot':'s22-art-review'})
 assert s['player']=='France' and s['campaign_journey']['status']=='active'
 info={'url':url,'pid':p.pid,'directory':str(run),'runtime_revision':pin,'executable_sha256':sha(exe),'player':s['player'],'date':s['date'],'build':b,
 'renderer_response_sha256':hashlib.sha256(asset).hexdigest(),'save_copy':{'source':str(saved),'path':str(target),'sha256':sha(target)},
 'scope':'Separate review runtime with repaired equipment shader. Copied S21 France campaign checkpoint; no original saves or review runtimes modified. S22 qualification remains pending.'}
 with (base/'S22-prep-review-launch.json').open('x',encoding='utf8') as f:json.dump(info,f,indent=2)
 print(json.dumps(info,indent=2))
except:
 p.terminate();raise
