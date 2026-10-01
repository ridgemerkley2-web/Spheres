"""One coordinated, missing-only source replay; stop immediately on HTTP 429."""
import datetime, hashlib, json, pathlib, re, subprocess, time
HERE=pathlib.Path(__file__).resolve().parent
ROOT=HERE.parents[4]
PRIOR=HERE.parent/'CLAUDE-C01-31'
EXTERNAL=pathlib.Path(r'D:\spheres-offload\codex-next-20260928\c01-31-original-responses-20261001')/'retrieval-04-resumed-0131'
EXTERNAL.mkdir(exist_ok=False)
sha=lambda b: hashlib.sha256(b).hexdigest()
now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
packet=json.loads((ROOT/'docs/campaign-certification/C01/research/japan.json').read_text(encoding='utf8'))
held=json.loads((PRIOR/'held-dependencies.json').read_text(encoding='utf8'))
sources={s['id']:s for s in packet['sources']}
report={'submission':'696937ba3d31280aab569cbd78418e9c3a0c6880','review_start':'d3b4e6daede2bc9fc571a88de1b0ba1049505b70','current_integration':subprocess.check_output(['git','rev-parse','509bd289'],cwd=ROOT,text=True).strip(),'prior_receipt':'../CLAUDE-C01-31/README.md','recipe':'One coordinated missing-only pass; exact recorded URL; curl without Accept-Encoding or auto-decompression; 10-second spacing; stop first HTTP429; original bodies/headers external, sanitized header derivatives public.','started_at':now(),'expected_missing':held['sources'],'results':[]}
def save():
    report['updated_at']=now()
    (HERE/'retrieval.json').write_text(json.dumps(report,indent=2,ensure_ascii=False)+'\n',encoding='utf8',newline='\n')
for index,sid in enumerate(held['sources']):
    s=sources[sid]
    extract=json.loads((ROOT/s['snapshot']['path']).read_text(encoding='utf8'))
    assert s['url']==extract['source_url'] and 'web.archive.org/web/' in s['url']
    body=EXTERNAL/(sid+'.bin'); headers=EXTERNAL/(sid+'.headers.txt')
    cmd=['curl.exe','--silent','--show-error','--connect-timeout','15','--max-time','50','--output',str(body),'--dump-header',str(headers),'--write-out','%{json}',s['url']]
    started=now(); response=subprocess.run(cmd,capture_output=True)
    raw=body.read_bytes() if body.exists() else b''
    metadata=json.loads(response.stdout.decode('utf8')) if response.stdout else {}
    h=headers.read_bytes() if headers.exists() else b''
    sanitized=re.sub(rb'(?im)^(set-cookie:)[^\r\n]*',rb'\1 [response value redacted]',h)
    derivative=HERE/(sid+'.headers.txt'); derivative.write_bytes(sanitized)
    retry=re.findall(rb'(?im)^retry-after:\s*([^\r\n]+)',h)
    row={'id':sid,'source_url':s['url'],'original_url':extract.get('original_url',s['url']),'archive_capture_utc':extract.get('archive_capture_utc'),'started_at':started,'finished_at':now(),'command':cmd,'exit_code':response.returncode,'http_status':metadata.get('http_code'),'curl_metadata':metadata,'stderr':response.stderr.decode('utf8',errors='replace'),'body_external':str(body),'bytes':len(raw),'sha256':sha(raw),'expected_bytes':extract['source_response_bytes'],'expected_sha256':extract['source_response_sha256'],'headers_external':str(headers),'headers_bytes':len(h),'headers_sha256':sha(h),'headers_derivative':derivative.name,'headers_derivative_bytes':len(sanitized),'headers_derivative_sha256':sha(sanitized),'retry_after':[x.decode('ascii',errors='replace') for x in retry]}
    row['exact']=response.returncode==0 and row['http_status']==200 and len(raw)==row['expected_bytes'] and row['sha256']==row['expected_sha256']
    report['results'].append(row)
    with (HERE/'attempts.jsonl').open('a',encoding='utf8',newline='\n') as out: out.write(json.dumps(row,ensure_ascii=False)+'\n')
    save()
    print(sid,'EXACT' if row['exact'] else 'UNVERIFIED',row['http_status'],'Retry-After:',row['retry_after'],flush=True)
    if row['http_status']==429:
        report['stopped_on_rate_limit']=sid
        break
    if index<len(held['sources'])-1: time.sleep(10)
report['finished_at']=now()
report['unattempted']=[sid for sid in held['sources'] if sid not in {r['id'] for r in report['results']}]
report['exact']=sum(r['exact'] for r in report['results'])
report['unavailable_or_different']=sum(not r['exact'] for r in report['results'])
save()
print(json.dumps({'exact':report['exact'],'unavailable_or_different':report['unavailable_or_different'],'unattempted':len(report['unattempted']),'stopped_on_rate_limit':report.get('stopped_on_rate_limit')}),flush=True)
