"""Retrieve only C01-40's recorded original responses; preserve every attempt."""
import argparse
import concurrent.futures
import datetime
import hashlib
import json
import pathlib
import subprocess
import time

BASE = 'f3e18e8306a0a7b1098b91f53f00efbb30da7990'
SUBMISSION = '54680e39910804b3864a3e973c452f2db2ac5e6f'
PACKET = 'docs/campaign-certification/C01/research/india.json'
ROOT = pathlib.Path(__file__).resolve().parents[6]
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--external', type=pathlib.Path, required=True)
parser.add_argument('--attempt', type=int, required=True)
args = parser.parse_args()
receipt = pathlib.Path(__file__).resolve().parents[1]
old = json.loads(subprocess.check_output(['git', 'show', BASE + ':' + PACKET], cwd=ROOT))
current = json.loads((ROOT / PACKET).read_text(encoding='utf-8'))
old_ids = {row['id'] for row in old['sources']}
sources = [row for row in current['sources'] if row['id'] not in old_ids]
assert len(sources) == 18
journal_path = receipt / 'source-attempts.json'
journal = json.loads(journal_path.read_text(encoding='utf-8')) if journal_path.exists() else {
    'format': 'spheres-c01-original-response-attempts/v1', 'base': BASE,
    'submission': SUBMISSION, 'external_root': str(args.external.resolve()),
    'recipe': 'curl.exe default User-Agent; no --compressed or Accept-Encoding override; original URL; maximum three redirects; received bytes retained unchanged',
    'attempts': [],
}
assert journal['external_root'] == str(args.external.resolve())
matched = {row['source_id'] for row in journal['attempts'] if row.get('exact')}
sources = [row for row in sources if row['id'] not in matched]
attempt_dir = args.external.resolve() / ('attempt-%02d' % args.attempt)
attempt_dir.mkdir(parents=True, exist_ok=False)


def fetch(source):
    extract = json.loads((ROOT / source['snapshot']['path']).read_text(encoding='utf-8'))
    stem = attempt_dir / source['id']
    body = stem.with_suffix('.body')
    headers = stem.with_suffix('.headers')
    stderr = stem.with_suffix('.stderr')
    command = ['curl.exe', '--silent', '--show-error', '--location', '--max-time', '45',
               '--connect-timeout', '15', '--max-redirs', '3', '--dump-header', str(headers),
               '--output', str(body), '--write-out', '%{http_code}\n%{url_effective}\n%{content_type}', source['url']]
    started = datetime.datetime.now(datetime.timezone.utc).isoformat()
    clock = time.monotonic()
    result = subprocess.run(command, capture_output=True)
    stderr.write_bytes(result.stderr)
    response = result.stdout.decode('utf-8', 'replace').splitlines()
    row = {'source_id': source['id'], 'attempt': args.attempt, 'url': source['url'],
           'started_utc': started, 'elapsed_seconds': round(time.monotonic() - clock, 3),
           'exit_code': result.returncode, 'http_status': response[0] if response else None,
           'effective_url': response[1] if len(response) > 1 else None,
           'content_type': response[2] if len(response) > 2 else None,
           'expected_bytes': extract['source_response_bytes'],
           'expected_sha256': extract['source_response_sha256'],
           'body_path': str(body), 'headers_path': str(headers), 'stderr_path': str(stderr),
           'error': result.stderr.decode('utf-8', 'replace'), 'exact': False}
    row['stderr_bytes'] = len(result.stderr)
    row['stderr_sha256'] = hashlib.sha256(result.stderr).hexdigest()
    if headers.exists():
        row['headers_sha256'] = hashlib.sha256(headers.read_bytes()).hexdigest()
    if body.exists():
        raw = body.read_bytes()
        row.update(bytes=len(raw), sha256=hashlib.sha256(raw).hexdigest())
        row['exact'] = (result.returncode == 0 and row['http_status'] == '200'
                        and row['bytes'] == row['expected_bytes']
                        and row['sha256'] == row['expected_sha256'])
    print(json.dumps({key: row.get(key) for key in ['source_id', 'http_status', 'exit_code', 'bytes', 'exact']}), flush=True)
    return row


with concurrent.futures.ThreadPoolExecutor(max_workers=3) as pool:
    journal['attempts'].extend(pool.map(fetch, sources))
journal['exact_source_count'] = len({row['source_id'] for row in journal['attempts'] if row['exact']})
journal_path.write_text(json.dumps(journal, indent=2) + '\n', encoding='utf-8')
print('EXACT', journal['exact_source_count'], '/18', flush=True)
