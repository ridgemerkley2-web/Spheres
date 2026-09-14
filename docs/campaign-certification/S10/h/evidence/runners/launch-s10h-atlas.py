import hashlib,json,pathlib,socket,subprocess,sys,time,urllib.request
BASE=pathlib.Path(__file__).resolve().parent;REPO=BASE/'integration';PIN=sys.argv[1]
def git(*args):return subprocess.check_output(['git','-c','core.longpaths=true',*args],cwd=REPO,text=True).strip()
assert git('rev-parse','HEAD')==PIN and not git('status','--porcelain')
OUT=BASE/'S10h-atlas-launch.json';assert not OUT.exists()
for port in range(7854,7900):
 with socket.socket() as s:
  try:s.bind(('127.0.0.1',port));break
  except OSError:continue
else:raise RuntimeError('No local review port available')
log=BASE/'S10h-atlas-server.log';assert not log.exists()
with log.open('xb') as stream:
 proc=subprocess.Popen([sys.executable,'-u',str(BASE/'serve-s10h-atlas.py'),str(port)],cwd=REPO,stdout=stream,stderr=subprocess.STDOUT,creationflags=subprocess.CREATE_NO_WINDOW|subprocess.DETACHED_PROCESS)
origin='http://127.0.0.1:'+str(port);files={}
for attempt in range(40):
 try:
  with urllib.request.urlopen(origin+'/tools/ui/leadership-research-review.html',timeout=2) as response:response.read()
  break
 except Exception:
  assert proc.poll() is None,'Review server exited';time.sleep(.15)
else:raise RuntimeError('Review server did not become ready')
for name in ['tools/ui/leadership-research-review.html','tools/ui/leadership-research-review.js','tools/ui/leadership-research-review.css','docs/campaign-certification/C01/research-index.json']:
 with urllib.request.urlopen(origin+'/'+name,timeout=5) as response:body=response.read()
 assert body==(REPO/name).read_bytes()
 files[name]=hashlib.sha256(body).hexdigest()
record={'passed':True,'source_revision':PIN,'url':origin+'/tools/ui/leadership-research-review.html?country=SouthAfrica','pid':proc.pid,'server_script_sha256':hashlib.sha256((BASE/'serve-s10h-atlas.py').read_bytes()).hexdigest(),'served_assets':files,'scope':'Local read-only static research review. No campaign server or save is loaded.'}
OUT.write_text(json.dumps(record,indent=2)+'\n',encoding='utf8',newline='\n');print(json.dumps(record,indent=2))
