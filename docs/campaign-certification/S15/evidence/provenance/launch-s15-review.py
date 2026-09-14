"""Launch a separate copied S15 review campaign after qualification."""
import datetime,hashlib,json,pathlib,shutil,socket,subprocess,sys,time,urllib.request
base=pathlib.Path(__file__).resolve().parent
def read(p):return json.loads(p.read_text(encoding='utf-8-sig'))
def sha(p):
 with open(p,'rb') as f:return hashlib.file_digest(f,'sha256').hexdigest()
pin=sys.argv[1];buildfile=pathlib.Path(sys.argv[2]);browserfile=pathlib.Path(sys.argv[3]);proof=read(buildfile);browser=read(browserfile)
assert proof['passed'] and browser['passed'] and proof['revision']==browser['build']['revision']==pin
source=pathlib.Path(proof['binary']['path']);assert sha(source)==proof['binary']['sha256']==browser['build']['binary_sha256']
saved=pathlib.Path(browser['run'])/'saves'/'s15-flight-operations.json';assert saved.is_file()
refs=list((browserfile.parent/'archive-audit').glob('*-s15-audit-continued.json'));assert len(refs)==1;reference=read(refs[0]);assert reference['canonical']['ignored_paths']==[]
run=base/('review-S15-'+datetime.datetime.now().strftime('%Y%m%d-%H%M%S'));run.mkdir();(run/'saves').mkdir()
exe=run/'spheres-web.exe';target=run/'saves/s15-flight-operations.json'
shutil.copy2(source,exe);shutil.copy2(saved,target);assert sha(source)==sha(exe) and sha(saved)==sha(target)
verify=run/'native-verification';verify.mkdir();worker=base/'integration/tools/ui/archive-worker.py'
check=subprocess.run([sys.executable,str(worker),str(target),str(verify/'copied-save.canonical'),'France'],capture_output=True,text=True,encoding='utf8',timeout=120,creationflags=subprocess.CREATE_NO_WINDOW)
assert check.returncode==0,check.stderr;audited=json.loads(check.stdout)
assert audited['canonical']['ignored_paths']==[] and audited['canonical']['sha256']==reference['canonical']['sha256'] and audited['canonical']['bytes']==reference['canonical']['bytes']
assert pathlib.Path(audited['canonical']['path']).read_bytes()==pathlib.Path(reference['canonical']['path']).read_bytes()
(verify/'copied-save.json').write_text(json.dumps(audited,indent=2)+'\n',encoding='utf8')
port=None
for candidate in range(7857,7875):
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
 state=request('/api/load',{'slot':'s15-flight-operations'})
 expected=read(browserfile.parent/'continued-state.json')
 assert state['player']=='France' and state['date']==expected['date'] and state['operations']==expected['operations']
 record={'url':url,'pid':process.pid,'directory':str(run),'runtime_revision':pin,'executable_sha256':sha(exe),'player':state['player'],'date':state['date'],'build':build,
  'save_copy':{'source':str(saved),'copy':str(target),'sha256':sha(target)},'native_verification':{'canonical_sha256':audited['canonical']['sha256'],'bytes':audited['canonical']['bytes'],'ignored_paths':[],'exact_bytes_match_final_browser':True,'worker_sha256':sha(worker)},
  'evidence':[{'path':str(p),'sha256':sha(p)} for p in [buildfile,browserfile]],'required_outcomes':browser['required_outcomes'],
  'scope':'Separate copy of the disclosed authored S15 France scenario. Existing review servers and original campaigns are preserved. This is a test campaign with manufactured preconditions, not a historical or unassisted campaign.'}
 with (base/'S15-review-launch.json').open('x',encoding='utf8') as f:json.dump(record,f,indent=2);f.write('\n')
 print(json.dumps(record,indent=2),flush=True)
except:
 process.terminate();raise

