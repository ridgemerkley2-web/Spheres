import datetime, hashlib, json, pathlib, subprocess, time

W = pathlib.Path('D:/spheres-offload/codex-next-20260928/review-ru28')
O = W.parent / 'ru28-originals-20261001'
O.mkdir(exist_ok=False)
(O / 'bodies').mkdir()
held = json.loads((W / 'docs/campaign-certification/C01/integrations/CLAUDE-C01-28/source-holds.json').read_text(encoding='utf8'))['sources']
assert len(held) == 12
rows = []
for i, s in enumerate(held):
    if i:
        time.sleep(10)
    source_id = s['source_id']
    body = O / 'bodies' / (source_id + '.bin')
    headers = body.with_suffix('.headers')
    started = datetime.datetime.now(datetime.timezone.utc).isoformat()
    result = subprocess.run(['curl.exe', '--silent', '--show-error', '--location', '--max-time', '45', '--max-redirs', '3', '--dump-header', str(headers), '--output', str(body), '--write-out', '%{http_code}\n%{url_effective}', s['url']], capture_output=True, text=True)
    response = result.stdout.splitlines()
    raw = body.read_bytes() if body.exists() else b''
    header_raw = headers.read_bytes() if headers.exists() else b''
    row = {'source_id': source_id, 'url': s['url'], 'started_utc': started, 'finished_utc': datetime.datetime.now(datetime.timezone.utc).isoformat(), 'exit_code': result.returncode, 'http_status': response[0] if response else '', 'effective_url': response[1] if len(response)>1 else '', 'stderr': result.stderr, 'expected_bytes': s['expected_bytes'], 'expected_sha256': s['expected_sha256'], 'body_path': str(body), 'body_exists': body.exists(), 'bytes': len(raw), 'sha256': hashlib.sha256(raw).hexdigest(), 'headers_path': str(headers), 'headers_bytes': len(header_raw), 'headers_sha256': hashlib.sha256(header_raw).hexdigest()}
    row['exact'] = row['http_status'] == '200' and row['bytes'] == row['expected_bytes'] and row['sha256'] == row['expected_sha256']
    row['rate_limit_stop'] = row['http_status'] == '429' or b'retry-after:' in header_raw.lower()
    rows.append(row)
    (O / 'retrieval.json').write_text(json.dumps({'reviewed_commit':'03141c43c5663e35d21eebc631aaf5eec4e909aa', 'prior_review_commit':'0dec078f747575f12eb851b15dc97602e2a06bfe', 'recipe':'One missing-only serial plain-curl pass; ten seconds between responses and next request; max45seconds and3redirects; stop on429 or Retry-After; no bypass or alternate hosts.', 'sources': rows, 'not_attempted': [x['source_id'] for x in held[len(rows):]]}, indent=2) + '\n', encoding='utf8')
    print(json.dumps({'source_id':source_id, 'http':row['http_status'], 'exact':row['exact'], 'bytes':row['bytes'], 'stop':row['rate_limit_stop']}), flush=True)
    if row['rate_limit_stop']:
        break
print('DONE', len(rows), 'attempts', sum(r['exact'] for r in rows), 'exact', flush=True)
