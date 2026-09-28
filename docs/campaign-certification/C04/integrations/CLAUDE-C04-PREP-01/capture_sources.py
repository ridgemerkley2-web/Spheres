"""Read-only recapture of the submission's 18 previously public/read source URLs.
Never retries a blocked source or treats a response hash as proof of a claim.
"""
import concurrent.futures,datetime,gzip,hashlib,json,pathlib,urllib.request,urllib.error
ROOT=pathlib.Path(__file__).resolve().parents[5]
HERE=pathlib.Path(__file__).resolve().parent
SOURCES=ROOT/'docs/campaign-certification/C04/preparation/france-tonga/sources.json'

def capture(source):
    row={'id':source['id'],'url':source['url'],'captured_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'submission_sha256':source.get('sha256'),'submission_bytes':source.get('bytes')}
    try:
        with urllib.request.urlopen(source['url'],timeout=35) as response:
            data=response.read(16*1024*1024+1)
            row.update(status=response.status,final_url=response.url,content_type=response.headers.get('Content-Type'))
            if len(data)>16*1024*1024: raise ValueError('Response exceeds bounded capture size')
        row.update(bytes=len(data),sha256=hashlib.sha256(data).hexdigest())
        row['submission_bytes_reproduced']=(row['bytes']==row['submission_bytes'] and row['sha256']==row['submission_sha256'])
        payload=gzip.compress(data,mtime=0)
        filename='source-bodies/'+source['id']+'.gz'
        (HERE/filename).parent.mkdir(exist_ok=True)
        (HERE/filename).write_bytes(payload)
        row.update(body=filename,gzip_bytes=len(payload),gzip_sha256=hashlib.sha256(payload).hexdigest())
    except Exception as error:
        row.update(error=str(error),status=getattr(error,'code',row.get('status')))
    return row

if __name__=='__main__':
    sources=json.loads(SOURCES.read_text(encoding='utf8'))
    items=[s for s in sources['sources'] if s['access_status']=='downloaded']
    with concurrent.futures.ThreadPoolExecutor(max_workers=4) as pool: rows=list(pool.map(capture,items))
    result={'reviewer':'Codex /root/s20_preflight','scope':'Independent response recapture, not proof of unreviewed content or edition currency. Original submission fetch records remain unchanged. No blocked original source was retried.','sources':rows}
    (HERE/'source-recapture.json').write_text(json.dumps(result,ensure_ascii=False,indent=2)+'\n',encoding='utf8')
    for r in rows: print(r['id'],r.get('status'),r.get('submission_bytes_reproduced'),r.get('error',''))
