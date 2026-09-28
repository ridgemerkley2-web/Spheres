import concurrent.futures,datetime,gzip,hashlib,json,pathlib,sys,time,urllib.request,urllib.error
root=pathlib.Path(r'D:/spheres-offload/codex-next-20260928/review-to24'); out=pathlib.Path(r'D:/spheres-offload/codex-next-20260928/evidence/to24-review')
sys.path.insert(0,str(root/'tools/avatars'))
import test_tonga_speakers_c01_24 as test
packet=json.loads((root/'docs/campaign-certification/C01/research/tonga.json').read_text(encoding='utf-8'))
sources=[s for s in packet['sources'] if s['id'] in test.NEW_SOURCES]
(out/'bodies').mkdir(exist_ok=True);(out/'headers').mkdir(exist_ok=True)
def sha(b):return hashlib.sha256(b).hexdigest()
def fetch(source):
 sid=source['id']; extract=json.loads((root/source['snapshot']['path']).read_text(encoding='utf-8'))
 row={'source_id':sid,'url':source['url'],'started_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'expected_bytes':extract['source_response_bytes'],'expected_sha256':extract['source_response_sha256'],'request_headers':{'Accept-Encoding':'identity','User-Agent':'Mozilla/5.0 (source-verification)'}}
 try:
  req=urllib.request.Request(row['url'],headers=row['request_headers'])
  with urllib.request.urlopen(req,timeout=35) as r:
   transfer=r.read(); headers=dict(r.headers);row.update(status=r.status,final_url=r.url)
  (out/'headers'/f'{sid}.json').write_text(json.dumps(headers,indent=2),encoding='utf-8')
  body=gzip.decompress(transfer) if transfer[:2]==b'\x1f\x8b' else transfer
  (out/'bodies'/f'{sid}.bin').write_bytes(body)
  if body!=transfer:(out/'bodies'/f'{sid}.transfer.bin').write_bytes(transfer)
  row.update(bytes=len(body),sha256=sha(body),transfer_bytes=len(transfer),transfer_sha256=sha(transfer),matched=len(body)==row['expected_bytes'] and sha(body)==row['expected_sha256'])
 except Exception as exc:row.update(matched=False,error=repr(exc))
 row['finished_utc']=datetime.datetime.now(datetime.timezone.utc).isoformat()
 return row
previous=json.loads((out/'retrieval-summary-01.json').read_text(encoding='utf-8'))
missing={r['source_id'] for r in previous['rows'] if not r['matched']}
sources=[s for s in sources if s['id'] in missing]
rows=[]
with concurrent.futures.ThreadPoolExecutor(max_workers=1) as pool:
 for row in pool.map(fetch,sources):
  rows.append(row)
  with (out/'retrieval-attempt-02.jsonl').open('a',encoding='utf-8',newline='\n') as f:f.write(json.dumps(row,ensure_ascii=False)+'\n')
  print(row['source_id'],row['matched'],row.get('bytes'),row.get('error',''),flush=True)
summary={'submission':'a32d44536b4ea290e57c8b51be2e5bf469c344d0','reviewer':'Codex /root/review_gap_submission','sources':len(rows),'exact_matches':sum(r['matched'] for r in rows),'rows':rows}
(out/'retrieval-summary-02.json').write_text(json.dumps(summary,ensure_ascii=False,indent=2)+'\n',encoding='utf-8',newline='\n')
print('SUMMARY',summary['sources'],summary['exact_matches'])
