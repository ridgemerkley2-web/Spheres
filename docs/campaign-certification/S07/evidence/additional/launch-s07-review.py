import datetime,hashlib,json,pathlib,shutil,socket,subprocess,sys,time,urllib.request
base=pathlib.Path(__file__).resolve().parent
repo=base/'integration'
pin='041007fbfda48cc027d0c24a1189705a1dcaa89b'
france,tonga=map(pathlib.Path,sys.argv[1:3])
for p,n in [(france,'France'),(tonga,'Tonga')]:
 r=json.loads(p.read_text());assert r['passed'] and r['nation']==n and r['build']['revision']==pin
source=base/'integration-target/release/spheres-web.exe'
sha=lambda p:hashlib.file_digest(p.open('rb'),'sha256').hexdigest()
assert sha(source)=='e845d3179f72e1ec49ae68197b48cad94dcfd0648ab5f358e0e9d28115d6d3c7'
port=7845
with socket.socket() as probe:probe.bind(('127.0.0.1',port))
run=base/('review-S07-'+datetime.datetime.now().strftime('%Y%m%d-%H%M%S'))
run.mkdir();(run/'saves').mkdir();exe=run/'spheres-web.exe';shutil.copy2(source,exe)
copies=[]
for p,n in [(france,'france'),(tonga,'tonga')]:
 for stage in ['paid','complete']:
  origin=p.parent/'saves'/f's07-{n}-{stage}.json';target=run/'saves'/origin.name;shutil.copy2(origin,target)
  assert sha(origin)==sha(target)
  copies.append(dict(source=str(origin),copy=str(target),sha256=sha(target)))
with (run/'server.stdout.log').open('wb') as out,(run/'server.stderr.log').open('wb') as err:
 proc=subprocess.Popen([str(exe),'--port',str(port),'--no-open'],cwd=run,stdout=out,stderr=err,creationflags=subprocess.CREATE_NO_WINDOW)
url=f'http://127.0.0.1:{port}'
def request(route,data=None):
 body=None if data is None else json.dumps(data).encode()
 req=urllib.request.Request(url+route,data=body,headers={'Content-Type':'application/json','Connection':'close'})
 with urllib.request.urlopen(req,timeout=10) as r:return json.load(r)
try:
 for attempt in range(100):
  assert proc.poll() is None
  try:build=request('/api/build');break
  except (OSError,TimeoutError):time.sleep(.1)
 else:raise RuntimeError('Review startup timeout')
 assert build['revision']==pin[:12]
 state=request('/api/load',{'slot':'s07-france-complete'})
 assert state['player']=='France'
 record=dict(url=url,pid=proc.pid,directory=str(run),runtime_revision=pin,executable_sha256=sha(exe),date=state['date'],player=state['player'],copies=copies,build=build)
 (run/'review-launch.json').write_text(json.dumps(record,indent=2)+'\n')
 (base/'S07-review-launch.json').write_text(json.dumps(record,indent=2)+'\n')
 print(json.dumps(record,indent=2))
except BaseException:
 proc.terminate();proc.wait(timeout=10);raise
