"""Independent C01-29 retrieval; raw bodies stay local, receipt records exact identity."""
import concurrent.futures, datetime, hashlib, json, pathlib, sys, urllib.error, urllib.request

root, output = map(pathlib.Path, sys.argv[1:])
output.mkdir(parents=True, exist_ok=False)
(output / 'bodies').mkdir()
packet = json.loads((root / 'docs/campaign-certification/C01/research/japan.json').read_text(encoding='utf-8'))
sources = packet['sources'][418:]
assert len(sources) == 54 and all(s['id'].startswith('jp_sdp_') for s in sources)

def retrieve(source):
    extract = json.loads((root / source['snapshot']['path']).read_text(encoding='utf-8'))
    row = dict(id=source['id'], url=source['url'], accessed_utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),
               expected_bytes=extract['source_response_bytes'], expected_sha256=extract['source_response_sha256'])
    try:
        request = urllib.request.Request(source['url'], headers={'Accept-Encoding': 'identity', 'User-Agent': 'SpheresResearchVerification/1.0'})
        with urllib.request.urlopen(request, timeout=35) as response:
            body = response.read()
            row.update(status=response.status, final_url=response.url, content_type=response.headers.get('Content-Type'), content_encoding=response.headers.get('Content-Encoding'))
        row.update(bytes=len(body), sha256=hashlib.sha256(body).hexdigest())
        row['exact'] = row['bytes'] == row['expected_bytes'] and row['sha256'] == row['expected_sha256']
        path = output / 'bodies' / (source['id'] + '.bin')
        path.write_bytes(body)
        row['body'] = str(path.relative_to(output)).replace('\\', '/')
    except Exception as exc:
        row.update(exact=False, error=str(exc))
    print(json.dumps({k:row[k] for k in ('id','status','exact','error') if k in row}), flush=True)
    return row

with concurrent.futures.ThreadPoolExecutor(max_workers=4) as pool:
    rows = list(pool.map(retrieve, sources))
report = dict(submission='e0bb7a01930b08a9b8b6d3b78caf3fc93af199d7', total=len(rows), retrieved=sum('sha256' in r for r in rows), exact=sum(r['exact'] for r in rows), sources=rows)
(output / 'retrieval.json').write_text(json.dumps(report, ensure_ascii=False, indent=2)+'\n', encoding='utf-8')
print(json.dumps({k:v for k,v in report.items() if k != 'sources'}))
