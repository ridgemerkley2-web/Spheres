import pathlib,subprocess,hashlib,json,datetime,shutil,socket,time,urllib.request,sys
base=pathlib.Path(__file__).resolve().parent;pin=sys.argv[1];label=sys.argv[2]
def read(p):return json.loads(pathlib.Path(p).read_text(encoding='utf-8-sig'))
def sha(p):
 with open(p,'rb') as f:return hashlib.file_digest(f,'sha256').hexdigest()
build=read(base/f'evidence/S21-{label}-binary.json');browser=read(base/f'evidence/S21-browser-{label}/result.json')
assert build['passed'] and browser['passed'] and build['revision']==browser['revision']==pin
source=base/'integration-target/release/spheres-web.exe';assert sha(source)==build['binary_sha256']==browser['binary_sha256']
old=read(base/'S18-review-launch.json');saved=pathlib.Path(old['directory'])/'saves/s18-flight-pages.json'
run=base/('review-S21-'+datetime.datetime.now().strftime('%Y%m%d-%H%M%S'));run.mkdir();(run/'saves').mkdir()
exe=run/'spheres-web.exe';target=run/'saves/s21-campaign.json';shutil.copy2(source,exe);shutil.copy2(saved,target)
assert sha(saved)==sha(target)
for port in range(7861,7890):
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
 s=req('/api/load',{'slot':'s21-campaign'})
 assert s['player']=='France' and s['campaign_journey']['status']=='active'
 info={'url':url,'pid':p.pid,'directory':str(run),'runtime_revision':pin,'executable_sha256':sha(exe),'player':s['player'],'date':s['date'],'build':b,'save_copy':{'source':str(saved),'path':str(target),'sha256':sha(target)},'scope':'Separate S21 review runtime, copied S18 France/Italy campaign checkpoint. Existing servers and saves remain in place. No late-date fixture is loaded here.'}
 with (base/'S21-review-launch.json').open('x',encoding='utf8') as f:json.dump(info,f,indent=2)
 print(json.dumps(info,indent=2))
except:
 p.terminate();raise
