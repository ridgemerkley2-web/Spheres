import concurrent.futures,datetime,hashlib,json,pathlib,subprocess,time
W=pathlib.Path('D:/spheres-offload/codex-next-20260928/review-in33');O=W.parent/'in33-review';O.mkdir(exist_ok=False);(O/'bodies').mkdir()
p='docs/campaign-certification/C01/research/india.json';data=json.loads((W/p).read_text(encoding='utf8'));old=json.loads(subprocess.check_output(['git','show','738e6610^:'+p],cwd=W));ids={s['id'] for s in old['sources']};sources=[s for s in data['sources'] if s['id'] not in ids];assert len(sources)==22
def get(s):
 e=json.loads((W/s['snapshot']['path']).read_text(encoding='utf8'));b=O/'bodies'/(s['id']+'.pdf');h=b.with_suffix('.headers');start=time.time()
 cmd=['curl.exe','--silent','--show-error','--location','--max-time','90','--max-redirs','3','--dump-header',str(h),'--output',str(b),'--write-out','%{http_code}\n%{url_effective}',s['url']]
 r=subprocess.run(cmd,capture_output=True,text=True);d={'source_id':s['id'],'url':s['url'],'started_utc':datetime.datetime.fromtimestamp(start,datetime.timezone.utc).isoformat(),'elapsed_seconds':time.time()-start,'exit_code':r.returncode,'response':r.stdout,'error':r.stderr,'expected_bytes':e['source_response_bytes'],'expected_sha256':e['source_response_sha256'],'body_path':str(b),'exact':False}
 if b.exists():
  raw=b.read_bytes();d.update(bytes=len(raw),sha256=hashlib.sha256(raw).hexdigest());d['exact']=d['bytes']==d['expected_bytes'] and d['sha256']==d['expected_sha256']
 print(json.dumps({'id':s['id'],'exact':d['exact'],'exit':r.returncode,'bytes':d.get('bytes')}),flush=True);return d
with concurrent.futures.ThreadPoolExecutor(max_workers=3) as pool:rows=list(pool.map(get,sources))
(O/'retrieval.json').write_text(json.dumps({'reviewed_commit':'2c78e89f67e9e21090a7da775fd8897c71d7dbd6','recipe':'plain curl without Accept-Encoding or automatic decoding; received PDF bytes retained','sources':rows},indent=2)+'\n',encoding='utf8')
print('EXACT',sum(r['exact'] for r in rows),'/',len(rows))
