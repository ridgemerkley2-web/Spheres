import concurrent.futures,datetime,hashlib,json,pathlib,subprocess
root=pathlib.Path(__file__).resolve().parents[6]
ev=pathlib.Path(__file__).parent
out=pathlib.Path(r"D:/spheres-offload/codex-next-20260928/c01-41-originals"); out.mkdir(parents=True,exist_ok=True)
p=json.loads((root/'docs/campaign-certification/C01/research/ussr.json').read_text('utf8'))
sources=p['sources'][-12:]
def fetch(s):
 e=json.loads((root/s['snapshot']['path']).read_text('utf8')); sid=s['id']; body=out/(sid+'.body'); header=ev/'headers'/(sid+'.txt'); header.parent.mkdir(exist_ok=True)
 started=datetime.datetime.now(datetime.timezone.utc).isoformat()
 cmd=['curl.exe','--silent','--show-error','--connect-timeout','30','--max-time','240','--header','Accept-Encoding: identity','--dump-header',str(header),'--output',str(body),'--write-out','%{http_code}',e['source_response_url']]
 r=subprocess.run(cmd,capture_output=True,text=True); raw=body.read_bytes() if body.exists() else b''
 result={'source_id':sid,'url':e['source_response_url'],'started_utc':started,'finished_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'curl_exit':r.returncode,'http_status':r.stdout,'stderr':r.stderr,'body_path':str(body),'bytes':len(raw),'sha256':hashlib.sha256(raw).hexdigest(),'expected_bytes':e['source_response_bytes'],'expected_sha256':e['source_response_sha256']}
 result['exact_match']=r.returncode==0 and r.stdout=='200' and result['bytes']==result['expected_bytes'] and result['sha256']==result['expected_sha256']; print(sid,result['http_status'],result['bytes'],result['exact_match'],flush=True); return result
with concurrent.futures.ThreadPoolExecutor(max_workers=4) as pool: rows=list(pool.map(fetch,sources))
(ev/'retrieval.json').write_text(json.dumps(rows,indent=2)+'\n',encoding='utf8')
