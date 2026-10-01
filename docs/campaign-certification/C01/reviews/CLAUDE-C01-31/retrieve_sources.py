"""Independently retrieve C01-31 originals; retain exact requests and failed attempts."""
import argparse
import datetime
import hashlib
import json
import pathlib
import subprocess
import time

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--repo', required=True, type=pathlib.Path)
parser.add_argument('--external', required=True, type=pathlib.Path)
parser.add_argument('--attempt', required=True)
parser.add_argument('--prior', action='append', default=[], type=pathlib.Path)
parser.add_argument('--interval', type=float, default=3.0)
args = parser.parse_args()
receipt_dir = pathlib.Path(__file__).resolve().parent
packet = json.loads((args.repo / 'docs/campaign-certification/C01/research/japan.json').read_text(encoding='utf8'))
sources = [s for s in packet['sources'] if s['id'].startswith('jp_komeito_')]
assert len(sources) == 31
done = set()
for prior in args.prior:
    done.update(row['id'] for row in json.loads(prior.read_text(encoding='utf8'))['results'] if row['exact'])
out = args.external / args.attempt
out.mkdir(parents=True, exist_ok=False)
checked = receipt_dir / args.attempt
checked.mkdir(parents=True, exist_ok=False)
results = []
now = lambda: datetime.datetime.now(datetime.timezone.utc).isoformat()
report = {'submission': '696937ba3d31280aab569cbd78418e9c3a0c6880', 'integration_base': 'f3e18e8306a0a7b1098b91f53f00efbb30da7990', 'reviewer': 'Codex /root/selector_leader_api', 'recipe': 'Recorded URL exactly; curl without Accept-Encoding header or automatic decompression; no alternate host, browser impersonation or authentication.', 'skipped_prior_exact': sorted(done), 'started_at': now(), 'results': results}
for source in sources:
    if source['id'] in done:
        continue
    extract = json.loads((args.repo / source['snapshot']['path']).read_text(encoding='utf8'))
    sid = source['id']
    assert extract['source_url'] == source['url']
    body = out / (sid + '.bin')
    headers = checked / (sid + '.headers.txt')
    command = ['curl.exe', '--silent', '--show-error', '--connect-timeout', '15', '--max-time', '50', '--output', str(body), '--dump-header', str(headers), '--write-out', '%{json}', source['url']]
    started = now()
    fetched = subprocess.run(command, capture_output=True)
    data = body.read_bytes() if body.exists() else b''
    meta = json.loads(fetched.stdout.decode('utf8')) if fetched.stdout else {}
    row = {'id': sid, 'source_url': source['url'], 'original_url': extract.get('original_url', source['url']), 'archive_capture_utc': extract.get('archive_capture_utc'), 'started_at': started, 'finished_at': now(), 'command': command, 'exit_code': fetched.returncode, 'http_status': meta.get('http_code'), 'effective_url': meta.get('url_effective'), 'curl_metadata': meta, 'stderr': fetched.stderr.decode('utf8', errors='replace'), 'body_external': str(body), 'headers': str(headers.relative_to(receipt_dir)), 'bytes': len(data), 'sha256': hashlib.sha256(data).hexdigest(), 'expected_bytes': extract['source_response_bytes'], 'expected_sha256': extract['source_response_sha256']}
    row['exact'] = fetched.returncode == 0 and row['http_status'] == 200 and row['bytes'] == row['expected_bytes'] and row['sha256'] == row['expected_sha256']
    results.append(row)
    with (checked / 'attempts.jsonl').open('a', encoding='utf8', newline='\n') as stream:
        stream.write(json.dumps(row, ensure_ascii=False) + '\n')
    report.update(exact=sum(r['exact'] for r in results), unavailable_or_different=sum(not r['exact'] for r in results), updated_at=now())
    (checked / 'retrieval.json').write_text(json.dumps(report, ensure_ascii=False, indent=2) + '\n', encoding='utf8')
    print(sid, 'EXACT' if row['exact'] else 'UNVERIFIED', row['http_status'], row['bytes'], flush=True)
    if row['http_status'] == 429:
        report['stopped_on_rate_limit'] = sid
        print('Rate limit observed; stopping this pass and retaining unattempted entries.', flush=True)
        break
    time.sleep(args.interval)
report['finished_at'] = now()
report['unattempted'] = [s['id'] for s in sources if s['id'] not in done and s['id'] not in {r['id'] for r in results}]
(checked / 'retrieval.json').write_text(json.dumps(report, ensure_ascii=False, indent=2) + '\n', encoding='utf8')
print(json.dumps({'exact': report['exact'], 'unavailable_or_different': report['unavailable_or_different'], 'skipped': len(done)}), flush=True)
