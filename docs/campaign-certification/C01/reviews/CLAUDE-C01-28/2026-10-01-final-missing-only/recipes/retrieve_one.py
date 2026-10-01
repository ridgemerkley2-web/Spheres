from pathlib import Path
import base64, datetime, hashlib, json, subprocess, sys, time

root=Path(__file__).resolve().parent
inventory=json.loads((root/'missing-inventory.json').read_bytes())
index=int(sys.argv[1])
log=root/'retrieval.json'
records=json.loads(log.read_bytes()) if log.exists() else []
assert len(records)==index, 'Only one normal request per source is permitted in this pass'
assert not any(r['stop_all_requests'] for r in records), 'Prior response requires stopping the pass'
if records:
    elapsed=(datetime.datetime.now(datetime.timezone.utc)-datetime.datetime.fromisoformat(records[-1]['finished_utc'])).total_seconds()
    if elapsed<15:time.sleep(15-elapsed)
item=inventory['items'][index];source=item['source'];extract=item['extract']
body=root/(source['id']+'.body');headers=root/(source['id']+'.headers')
assert not body.exists() and not headers.exists()
command=['curl.exe','--silent','--show-error','--location','--max-time','45','--max-redirs','3','--dump-header',str(headers),'--output',str(body),'--write-out','%{http_code}\n%{url_effective}',source['url']]
started=datetime.datetime.now(datetime.timezone.utc).isoformat()
run=subprocess.run(command,stdout=subprocess.PIPE,stderr=subprocess.PIPE)
finished=datetime.datetime.now(datetime.timezone.utc).isoformat()
output=run.stdout.decode('utf-8','replace').splitlines();raw=body.read_bytes() if body.exists() else b'';head=headers.read_bytes() if headers.exists() else b''
status=output[0] if output else ''
record={'source_id':source['id'],'requested_url':source['url'],'command':command,'started_utc':started,'finished_utc':finished,'exit_code':run.returncode,'http_status':status,'effective_url':output[1] if len(output)>1 else None,'stderr':run.stderr.decode('utf-8','replace'),'body_path':body.name,'body_exists':body.exists(),'bytes':len(raw),'sha256':hashlib.sha256(raw).hexdigest(),'sha1_base32':base64.b32encode(hashlib.sha1(raw).digest()).decode().rstrip('='),'headers_path':headers.name,'headers_bytes':len(head),'headers_sha256':hashlib.sha256(head).hexdigest(),'expected_bytes':extract['source_response_bytes'],'expected_sha256':extract['source_response_sha256'],'stop_all_requests':status in ('429','503') or b'retry-after:' in head.lower()}
record['exact']=status=='200' and run.returncode==0 and record['bytes']==record['expected_bytes'] and record['sha256']==record['expected_sha256']
records.append(record);log.write_text(json.dumps(records,indent=2)+'\n',encoding='utf-8',newline='\n')
print(json.dumps(record),flush=True)
