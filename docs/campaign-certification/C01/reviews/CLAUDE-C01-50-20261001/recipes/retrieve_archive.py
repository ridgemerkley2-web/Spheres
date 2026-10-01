import json,pathlib,subprocess,hashlib,datetime,time
root=pathlib.Path(r'D:/spheres-offload/codex-next-20260928/claude-review-20261001-04/saudi-evidence');records=[]
for source in json.loads((root/'new-sources.json').read_text(encoding='utf-8-sig')):
 if 'web.archive.org' not in source['url']:continue
 if records: time.sleep(15)
 stem=source['id'];body=root/(stem+'.body');headers=root/(stem+'.headers.txt');cmd=['curl.exe','--silent','--show-error','--location','--max-time','55','--dump-header',str(headers),'--output',str(body),'--write-out','%{http_code}|%{url_effective}',source['url']]
 r={'source_id':stem,'requested_url':source['url'],'retrieved_at':datetime.datetime.now(datetime.timezone.utc).isoformat(),'command':cmd};p=subprocess.run(cmd,capture_output=True,text=True);r.update(returncode=p.returncode,response=p.stdout,stderr=p.stderr)
 for key,path in [('body',body),('headers',headers)]:
  if path.exists():r[key]={'path':str(path),'bytes':path.stat().st_size,'sha256':hashlib.sha256(path.read_bytes()).hexdigest()}
 records.append(r);(root/'archive-retrieval.json').write_text(json.dumps(records,ensure_ascii=False,indent=2)+'\n',encoding='utf-8',newline='\n');print(stem,p.stdout,r.get('body'),flush=True)
 if p.stdout.startswith(('429','503')):break
