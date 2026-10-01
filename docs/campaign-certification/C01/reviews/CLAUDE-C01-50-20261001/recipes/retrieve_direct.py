import json,pathlib,subprocess,hashlib,datetime,time
root=pathlib.Path(r'D:/spheres-offload/codex-next-20260928/claude-review-20261001-04/saudi-evidence')
sources=json.loads((root/'new-sources.json').read_text(encoding='utf-8-sig'))
records=[]
for source in sources:
 if 'portalapi.spa.gov.sa' not in source['url']: continue
 stem=source['id']; body=root/(stem+'.body'); headers=root/(stem+'.headers.txt')
 command=['curl.exe','--silent','--show-error','--location','--max-time','55','--dump-header',str(headers),'--output',str(body),'--write-out','%{http_code}|%{url_effective}',source['url']]
 start=datetime.datetime.now(datetime.timezone.utc).isoformat()
 result=subprocess.run(command,capture_output=True,text=True)
 record={'source_id':stem,'requested_url':source['url'],'retrieved_at':start,'command':command,'returncode':result.returncode,'response':result.stdout,'stderr':result.stderr}
 for key,path in [('body',body),('headers',headers)]:
  if path.exists(): record[key]={'path':str(path),'bytes':path.stat().st_size,'sha256':hashlib.sha256(path.read_bytes()).hexdigest()}
 if result.stdout.startswith('200'):
  try:
   data=json.loads(body.read_bytes())['data'];content=data['content'];(root/(stem+'.content.txt')).write_text(content,encoding='utf-8',newline='\n')
   record['content_sha256']=hashlib.sha256(content.encode()).hexdigest();record['published_at']=data.get('published_at');record['data_keys']=list(data)
  except Exception as e: record['parse_error']=str(e)
 records.append(record);(root/'direct-retrieval.json').write_text(json.dumps(records,ensure_ascii=False,indent=2)+'\n',encoding='utf-8',newline='\n')
 print(stem,result.returncode,result.stdout,record.get('body',{}).get('sha256'),flush=True)
 if result.stdout.startswith(('429','503')):break
 time.sleep(2)
