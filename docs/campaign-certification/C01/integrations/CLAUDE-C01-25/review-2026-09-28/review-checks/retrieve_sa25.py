"""Independent submission retrieval preparation; original bodies remain local."""
import concurrent.futures, datetime, hashlib, json, pathlib, subprocess, sys, urllib.request

root, output = map(pathlib.Path, sys.argv[1:])
output.mkdir(parents=True, exist_ok=False)
(output / 'bodies').mkdir()
revision = subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=root, text=True).strip()
assert revision == '78e52b03cb2f321ed30e621fe0512e92b2e1e579'
packet = json.loads((root / 'docs/campaign-certification/C01/research/saudi-arabia.json').read_text(encoding='utf-8'))
sources = packet['sources'][32:]
assert len(sources) == 71 and len({s['id'] for s in sources}) == 71

def retrieve(source):
    extract = json.loads((root / source['snapshot']['path']).read_text(encoding='utf-8'))
    row = dict(id=source['id'], url=source['url'], accessed_utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),
               expected_bytes=extract['source_response_bytes'], expected_sha256=extract['source_response_sha256'])
    try:
        request = urllib.request.Request(source['url'], headers={'Accept-Encoding': 'identity', 'User-Agent': 'curl/8.16.0'})
        with urllib.request.urlopen(request, timeout=35) as response:
            body = response.read(30_000_001)
            assert len(body) <= 30_000_000, 'Response exceeds declared 30MB safety bound'
            row.update(status=response.status, final_url=response.url, content_type=response.headers.get('Content-Type'), content_encoding=response.headers.get('Content-Encoding'))
        row.update(bytes=len(body), sha256=hashlib.sha256(body).hexdigest())
        row['exact'] = row['bytes'] == row['expected_bytes'] and row['sha256'] == row['expected_sha256']
        path = output / 'bodies' / (source['id'] + '.bin')
        path.write_bytes(body)
        row['body'] = path.relative_to(output).as_posix()
    except Exception as exc:
        row.update(exact=False, error=str(exc))
    print(json.dumps({k:row[k] for k in ('id','status','exact','error') if k in row}), flush=True)
    return row

with concurrent.futures.ThreadPoolExecutor(max_workers=4) as pool:
    rows = list(pool.map(retrieve, sources))
report = dict(submission=revision, user_agent='curl/8.16.0', accept_encoding='identity', scope='Response reproduction only; no content acceptance implied', total=len(rows), retrieved=sum('sha256' in r for r in rows), exact=sum(r['exact'] for r in rows), sources=rows)
(output / 'retrieval.json').write_text(json.dumps(report, ensure_ascii=False, indent=2)+'\n', encoding='utf-8')
print(json.dumps({k:v for k,v in report.items() if k != 'sources'}))
