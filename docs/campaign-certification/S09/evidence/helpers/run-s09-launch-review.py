"""Launch a separate S09 review from its verified browser-saved France draft."""
import datetime,hashlib,json,pathlib,shutil,socket,subprocess,sys,time,urllib.request
base=pathlib.Path(__file__).resolve().parent
def sha(p):
 with p.open('rb') as f:return hashlib.file_digest(f,'sha256').hexdigest()
def same_bytes(left,right):
 with left.open('rb') as a,right.open('rb') as b:
  while True:
   x=a.read(1024*1024);y=b.read(1024*1024)
   if x!=y:return False
   if not x:return True

def main():
 pin=sys.argv[1];buildfile=pathlib.Path(sys.argv[2]).resolve();browserfile=pathlib.Path(sys.argv[3]).resolve()
 read=lambda p:json.loads(p.read_text(encoding='utf-8-sig'))
 buildproof,browser=map(read,[buildfile,browserfile])
 assert buildproof['passed'] and buildproof['revision_before']==pin==buildproof['revision_after']
 assert browser['passed'] and browser['build']['revision']==pin
 assert browser['build']['binary_sha256']==buildproof['binary_sha256'],'Browser and launch executable differ'
 source=pathlib.Path(buildproof['binary']).resolve();assert sha(source)==buildproof['binary_sha256']
 saved=pathlib.Path(browser['run'])/'saves/s09-saved-design.json';assert saved.is_file()
 audit_dir=pathlib.Path(browser['run']).resolve().parent/'archive-audit'
 audit_files=list(audit_dir.glob('*-s09-after-continue.json'))
 assert len(audit_files)==1,'Require the final browser Continue native audit'
 audit_file=audit_files[0];final_audit=read(audit_file)
 assert final_audit['requested']['buyer']=='France' and final_audit['canonical']['ignored_paths']==[]
 audited_raw=pathlib.Path(final_audit['input']['path']).resolve();audited_canonical=pathlib.Path(final_audit['canonical']['path']).resolve()
 assert audited_raw.is_relative_to(pathlib.Path(browser['run']).resolve()/'audit-captures')
 assert audited_canonical.parent==audit_dir.resolve()
 assert sha(audited_raw)==final_audit['input']['sha256'],'Browser-audited native capture changed'
 assert audited_canonical.stat().st_size==final_audit['canonical']['bytes'] and sha(audited_canonical)==final_audit['canonical']['sha256'],'Browser canonical evidence changed'
 port=None
 for candidate in range(7847,7861):
  with socket.socket() as probe:
   try:probe.bind(('127.0.0.1',candidate));port=candidate;break
   except OSError:continue
 assert port is not None
 run=base/('review-S09-'+datetime.datetime.now().strftime('%Y%m%d-%H%M%S'));run.mkdir();(run/'saves').mkdir()
 exe=run/'spheres-web.exe';target=run/'saves/s09-saved-design.json';shutil.copy2(source,exe);shutil.copy2(saved,target)
 assert sha(source)==sha(exe) and sha(saved)==sha(target)
 verification=run/'native-verification';verification.mkdir()
 worker=base/'integration/tools/ui/archive-worker.py';worker_hash=sha(worker)
 checked=subprocess.run([sys.executable,str(worker),str(target),str(verification/'copied-save.canonical'),'France'],capture_output=True,text=True,encoding='utf-8',timeout=120,creationflags=subprocess.CREATE_NO_WINDOW)
 assert checked.returncode==0,'Copied save audit failed: '+checked.stderr[:4000]
 copy_audit=json.loads(checked.stdout)
 with (verification/'copied-save.json').open('x',encoding='utf-8') as f:json.dump(copy_audit,f,indent=2);f.write(chr(10))
 assert sha(worker)==worker_hash and copy_audit['input']['sha256']==sha(saved)==sha(target)
 assert copy_audit['requested']['buyer']=='France' and copy_audit['canonical']['ignored_paths']==[]
 assert copy_audit['canonical']['format']==final_audit['canonical']['format']
 assert copy_audit['canonical']['bytes']==final_audit['canonical']['bytes'] and copy_audit['canonical']['sha256']==final_audit['canonical']['sha256'],'Named save differs from the final browser native world'
 assert same_bytes(pathlib.Path(copy_audit['canonical']['path']),audited_canonical),'Native canonical bytes differ'
 native_proof={'browser_audit':{'path':str(audit_file),'sha256':sha(audit_file)},'copied_save_audit':{'path':str(verification/'copied-save.json'),'sha256':sha(verification/'copied-save.json')},'worker':{'path':str(worker),'sha256':worker_hash},'canonical_bytes':copy_audit['canonical']['bytes'],'canonical_sha256':copy_audit['canonical']['sha256'],'ignored_paths':[],'exact_canonical_bytes_equal':True}
 with (run/'server.stdout.log').open('xb') as out,(run/'server.stderr.log').open('xb') as err:
  proc=subprocess.Popen([str(exe),'--port',str(port),'--no-open'],cwd=run,stdout=out,stderr=err,creationflags=subprocess.CREATE_NO_WINDOW)
 url=f'http://127.0.0.1:{port}'
 def request(route,data=None):
  req=urllib.request.Request(url+route,data=None if data is None else json.dumps(data).encode(),headers={'Content-Type':'application/json','Connection':'close'})
  with urllib.request.urlopen(req,timeout=30) as r:return json.load(r)
 try:
  deadline=time.monotonic()+20
  while True:
   assert proc.poll() is None
   try:build=request('/api/build');break
   except OSError:
    if time.monotonic()>deadline:raise
    time.sleep(.1)
  assert build['revision']==pin[:12]
  state=request('/api/load',{'slot':'s09-saved-design'});assert state['player']=='France'
  board=request('/api/equipment?session_id='+state['session_id'])
  stored=next(r for r in board['designs'] if r['id']=='draft:S09 Atlas')
  assert stored['spec']==browser['saved_draft']['spec'] and stored['name']==browser['saved_draft']['name']
  record={'url':url,'pid':proc.pid,'directory':str(run),'runtime_revision':pin,'executable_sha256':sha(exe),'date':state['date'],'player':state['player'],'saved_draft':stored,
   'save_copy':{'source':str(saved),'copy':str(target),'sha256':sha(target)},'build':build,'native_verification':native_proof,
   'qualification':[{'path':str(p),'sha256':sha(p)} for p in [buildfile,browserfile]],
   'scope':'Separate review with the browser-proven saved design. Existing reviews and original campaigns retained.'}
  for dest in [run/'review-launch.json',base/'S09-review-launch.json']:
   with dest.open('x',encoding='utf-8') as f:json.dump(record,f,indent=2);f.write('\n')
  print(json.dumps(record,indent=2),flush=True)
 except BaseException:
  proc.terminate();proc.wait(timeout=10);raise

if __name__=='__main__':main()
