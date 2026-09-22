import pathlib,subprocess,hashlib,json,datetime,shutil,socket,time,urllib.request,sys
base=pathlib.Path(__file__).resolve().parent
repo=base/sys.argv[1];binary=base/sys.argv[2];pin=sys.argv[3];label=sys.argv[4];first_port=int(sys.argv[5])
def sha(p):
 with p.open('rb') as f:return hashlib.file_digest(f,'sha256').hexdigest()
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=repo,text=True).strip()==pin
assert not subprocess.check_output(['git','status','--porcelain'],cwd=repo,text=True).strip()
saved=base/'evidence/S19-review-01/candidate-navigation/server/saves/s19-route-review.json'
run=base/('review-'+label+'-'+datetime.datetime.now().strftime('%Y%m%d-%H%M%S'));run.mkdir();(run/'saves').mkdir()
exe=run/'spheres-web.exe';target=run/'saves/development-review.json';shutil.copy2(binary,exe);shutil.copy2(saved,target)
for port in range(first_port,7900):
 with socket.socket() as probe:
  try:probe.bind(('127.0.0.1',port));break
  except OSError:continue
else:raise RuntimeError('No free preview port')
with (run/'stdout.log').open('xb') as out,(run/'stderr.log').open('xb') as err:
 process=subprocess.Popen([str(exe),'--port',str(port),'--no-open'],cwd=run,stdout=out,stderr=err,creationflags=subprocess.CREATE_NO_WINDOW)
url=f'http://127.0.0.1:{port}'
def req(route,data=None):
 q=urllib.request.Request(url+route,data=None if data is None else json.dumps(data).encode(),headers={'Content-Type':'application/json'})
 with urllib.request.urlopen(q,timeout=30) as r:return json.load(r)
try:
 until=time.monotonic()+30
 while True:
  assert process.poll() is None
  try:build=req('/api/build');break
  except OSError:
   if time.monotonic()>until:raise
   time.sleep(.1)
 assert build['revision']==pin[:12]
 assets={}
 for name in ['index.html','equipment-ui.js','equipment-ui.css','province-economy-ui.js','province-economy.css','performance-ui.js','guidance-ui.js','advisor-model.js','equipment-model.js']:
  with urllib.request.urlopen(url+('/' if name=='index.html' else '/'+name),timeout=30) as r:body=r.read()
  assert body==(repo/'spheres-web/ui'/name).read_bytes(),name+' embedded bytes differ'
  assets[name]=hashlib.sha256(body).hexdigest()
 state=req('/api/load',{'slot':'development-review'});assert state['player']=='France' and state['date']=='3 Jan 1990'
 info={'url':url,'pid':process.pid,'directory':str(run),'runtime_revision':pin,'executable_sha256':sha(exe),'build':build,'served_assets_sha256':assets,
  'player':state['player'],'date':state['date'],'save_copy':{'path':str(target),'source':str(saved),'sha256':sha(target)},
  'scope':'Separate development review runtime. Exact compiled assets; original playsets and saves preserved. No campaign certification.'}
 with (base/(label+'-launch.json')).open('x',encoding='utf8') as f:json.dump(info,f,indent=2)
 print(json.dumps(info,indent=2))
except:
 process.terminate();raise
