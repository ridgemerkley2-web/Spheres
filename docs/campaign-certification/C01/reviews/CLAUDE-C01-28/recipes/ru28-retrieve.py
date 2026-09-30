import datetime,hashlib,json,pathlib,subprocess,time,sys
W=pathlib.Path('D:/spheres-offload/codex-next-20260928/review-ru28');O=W.parent/'ru28-review'
retry='--retry' in sys.argv
if not retry: O.mkdir(exist_ok=False);(O/'bodies').mkdir()
p='docs/campaign-certification/C01/research/russia.json'
d=json.loads((W/p).read_text(encoding='utf8'));old=json.loads(subprocess.check_output(['git','show','cc06f873^:'+p],cwd=W))
ids={s['id'] for s in old['sources']};sources=[s for s in d['sources'] if s['id'] not in ids];assert len(sources)==68
if retry:
    passed={s['source_id'] for s in json.loads((O/'retrieval.json').read_text())['sources'] if s['exact']}
    sources=[s for s in sources if s['id'] not in passed]
rows=[];dest='retrieval-retry.json' if retry else 'retrieval.json'
assert not (O/dest).exists()
for s in sources:
    e=json.loads((W/s['snapshot']['path']).read_text(encoding='utf8'))
    b=O/'bodies'/(s['id']+('.retry' if retry else '')+'.bin');h=b.with_suffix('.headers');start=time.time()
    u=e['source_response_url']
    r=subprocess.run(['curl.exe','--silent','--show-error','--location','--max-time','60','--max-redirs','3','--dump-header',str(h),'--output',str(b),'--write-out','%{http_code}\n%{url_effective}',u],capture_output=True,text=True)
    row={'source_id':s['id'],'url':u,'started_utc':datetime.datetime.fromtimestamp(start,datetime.timezone.utc).isoformat(),'elapsed_seconds':time.time()-start,'exit_code':r.returncode,'response':r.stdout,'error':r.stderr,'expected_bytes':e['source_response_bytes'],'expected_sha256':e['source_response_sha256'],'body_path':str(b),'exact':False}
    if b.exists():
        raw=b.read_bytes();row.update(bytes=len(raw),sha256=hashlib.sha256(raw).hexdigest())
        row['exact']=row['bytes']==row['expected_bytes'] and row['sha256']==row['expected_sha256']
    rows.append(row)
    (O/dest).write_text(json.dumps({'reviewed_commit':'03141c43c5663e35d21eebc631aaf5eec4e909aa','recipe':'Sequential plain curl, no Accept-Encoding or decoding, 60s timeout, max3redirects; one missing-only retry permitted separately.','sources':rows},indent=2)+'\n',encoding='utf8')
    print(json.dumps({'id':s['id'],'exact':row['exact'],'exit':r.returncode,'bytes':row.get('bytes')}),flush=True)
print('EXACT',sum(r['exact'] for r in rows),'/',len(rows))
