import datetime,hashlib,json,pathlib,shutil,socket,subprocess,sys,time,urllib.request
base=pathlib.Path(__file__).resolve().parent
def sha(p):
 with open(p,'rb') as f:return hashlib.file_digest(f,'sha256').hexdigest()
def read(p):return json.loads(p.read_text(encoding='utf-8-sig'))
pin=sys.argv[1];buildfile=base/'evidence/S10-f-binary.json';browserfile=pathlib.Path(sys.argv[2])
proof,browser=read(buildfile),read(browserfile)
assert proof['passed'] and browser['passed'] and proof['revision']==browser['build']['revision']==pin
source=pathlib.Path(proof['binary']['path']);assert sha(source)==proof['binary']['sha256']==browser['build']['binary_sha256']
saved=pathlib.Path(browser['run'])/'saves/s10-government-decisions.json';assert saved.is_file()
audits=browserfile.parent/'archive-audit';matches=list(audits.glob('*-s10-after-continue.json'));assert len(matches)==1
reference=read(matches[0]);assert reference['canonical']['ignored_paths']==[]
run=base/('review-S10f-'+datetime.datetime.now().strftime('%Y%m%d-%H%M%S'));run.mkdir();(run/'saves').mkdir()
exe=run/'spheres-web.exe';target=run/'saves/s10-government-decisions.json'
shutil.copy2(source,exe);shutil.copy2(saved,target);assert sha(source)==sha(exe) and sha(saved)==sha(target)
verify=run/'native-verification';verify.mkdir()
worker=base/'integration/tools/ui/archive-worker.py'
checked=subprocess.run([sys.executable,str(worker),str(target),str(verify/'copied-save.canonical'),'France'],capture_output=True,text=True,encoding='utf-8',timeout=120,creationflags=subprocess.CREATE_NO_WINDOW)
assert checked.returncode==0,checked.stderr[:2000]
audited=json.loads(checked.stdout)
assert audited['canonical']['ignored_paths']==[] and audited['canonical']['sha256']==reference['canonical']['sha256']
assert audited['canonical']['bytes']==reference['canonical']['bytes']
assert pathlib.Path(audited['canonical']['path']).read_bytes()==pathlib.Path(reference['canonical']['path']).read_bytes()
(verify/'copied-save.json').write_text(json.dumps(audited,indent=2)+'\n',encoding='utf-8')
port=None
for candidate in range(7852,7863):
 with socket.socket() as probe:
  try:probe.bind(('127.0.0.1',candidate));port=candidate;break
  except OSError:continue
assert port
with (run/'server.stdout.log').open('xb') as out,(run/'server.stderr.log').open('xb') as err:
 process=subprocess.Popen([str(exe),'--port',str(port),'--no-open'],cwd=run,stdout=out,stderr=err,creationflags=subprocess.CREATE_NO_WINDOW)
url=f'http://127.0.0.1:{port}'
def request(route,data=None):
 req=urllib.request.Request(url+route,data=None if data is None else json.dumps(data).encode(),headers={'Content-Type':'application/json','Connection':'close'})
 with urllib.request.urlopen(req,timeout=30) as response:return json.load(response)
try:
 until=time.monotonic()+20
 while True:
  assert process.poll() is None
  try:build=request('/api/build');break
  except OSError:
   if time.monotonic()>until:raise
   time.sleep(.1)
 assert build['revision']==pin[:12]
 state=request('/api/load',{'slot':'s10-government-decisions'})
 assert state['player']=='France' and state['agency']['policy']==browser['final_state']['policy']
 gov=request('/api/government?nation=France')
 assert gov['leader']==browser['final_state']['leader'] and gov['political_capital']==browser['final_state']['political_capital']
 record={'url':url,'pid':process.pid,'directory':str(run),'runtime_revision':pin,'executable_sha256':sha(exe),'player':state['player'],'date':state['date'],'policy':state['agency']['policy'],'build':build,
  'save_copy':{'source':str(saved),'copy':str(target),'sha256':sha(target)},'native_verification':{'canonical_sha256':audited['canonical']['sha256'],'bytes':audited['canonical']['bytes'],'ignored_paths':[],'exact_bytes_match_final_browser':True,'worker_sha256':sha(worker)},
  'evidence':[{'path':str(p),'sha256':sha(p)} for p in [buildfile,browserfile]],'scope':'Independent review; previous review servers and original campaigns preserved.'}
 (base/'S10f-review-launch.json').write_text(json.dumps(record,indent=2)+'\n',encoding='utf-8')
 print(json.dumps(record,indent=2),flush=True)
except:
 process.terminate();raise
