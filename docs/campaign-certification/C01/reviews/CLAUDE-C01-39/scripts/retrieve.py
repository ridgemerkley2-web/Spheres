"""Single paced C01-39 original-response pass; stop immediately on HTTP 429."""
import argparse
import base64
import datetime
import hashlib
import json
import pathlib
import subprocess
import time

ROOT = pathlib.Path(__file__).resolve().parents[6]
RECEIPT = pathlib.Path(__file__).resolve().parents[1]
BASE = '509bd2890f71850c97215d304ff16b757c7d49c2'
TIP = '9cb02c20012a6e6314d7e64d52241609d566f738'
PACKET = 'docs/campaign-certification/C01/research/south-africa.json'
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--external', type=pathlib.Path, required=True)
parser.add_argument('--network-slot-released', action='store_true')
parser.add_argument('--pace-seconds', type=int, default=10)
args = parser.parse_args()
assert args.network_slot_released, 'Await the coordinating reviewer releasing the Archive network slot.'
assert args.pace_seconds >= 10
old = json.loads(subprocess.check_output(['git', 'show', BASE + ':' + PACKET], cwd=ROOT))
new = json.loads((ROOT / PACKET).read_text(encoding='utf-8'))
old_ids = {s['id'] for s in old['sources']}
sources = [s for s in new['sources'] if s['id'] not in old_ids]
assert len(sources) == 24
# Read the explicitly requested vacancy question and holder dependencies first;
# every source still receives at most one request in this pass.
priority = ['za_da_fedcouncil_chair_leadership_vacancies_20191024',
            'za_da_fedcouncil_chair_interim_election_20191025']
da = next(o for o in new['organizations'] if o['id'] == 'za_iec_n2024_027')
role = next(r for r in da['roles'] if r['id'] == 'za_da_federal_leader')
for holder in role['holder_claims']:
    for sid in holder['sources']:
        if sid not in old_ids and sid not in priority:
            priority.append(sid)
sources.sort(key=lambda s: priority.index(s['id']) if s['id'] in priority else len(priority))
out = args.external.resolve() / 'attempt-01'
out.mkdir(parents=True, exist_ok=False)
journal = {'format': 'spheres-c01-original-response-attempts/v1', 'base': BASE, 'submission': TIP,
           'external_root': str(args.external.resolve()),
           'recipe': 'Single sequential pass, default curl User-Agent, no Accept-Encoding or --compressed, recorded URL unchanged; ten-second minimum gap; stop on first HTTP 429, no bypass or retry.',
           'request_order': [s['id'] for s in sources],
           'attempts': [], 'not_attempted': []}
journal_path = RECEIPT / 'source-attempts.json'


def save():
    journal['exact_source_count'] = sum(row['exact'] for row in journal['attempts'])
    journal_path.write_text(json.dumps(journal, indent=2) + '\n', encoding='utf-8', newline='\n')


for index, source in enumerate(sources):
    if index:
        time.sleep(args.pace_seconds)
    extract = json.loads((ROOT / source['snapshot']['path']).read_text(encoding='utf-8'))
    stem = out / source['id']
    body, headers, stderr = [stem.with_suffix('.' + suffix) for suffix in ('body', 'headers', 'stderr')]
    command = ['curl.exe', '--silent', '--show-error', '--location', '--max-redirs', '3',
               '--max-time', '45', '--connect-timeout', '15', '--dump-header', str(headers),
               '--output', str(body), '--write-out', '%{http_code}\n%{url_effective}\n%{content_type}', source['url']]
    started = datetime.datetime.now(datetime.timezone.utc).isoformat()
    clock = time.monotonic()
    response = subprocess.run(command, capture_output=True)
    stderr.write_bytes(response.stderr)
    info = response.stdout.decode('utf-8', 'replace').splitlines()
    row = {'source_id': source['id'], 'attempt': 1, 'url': source['url'], 'started_utc': started,
           'elapsed_seconds': round(time.monotonic() - clock, 3), 'exit_code': response.returncode,
           'http_status': info[0] if info else None, 'effective_url': info[1] if len(info) > 1 else None,
           'content_type': info[2] if len(info) > 2 else None, 'expected_bytes': extract['source_response_bytes'],
           'expected_sha256': extract['source_response_sha256'],
           'declared_cdx_sha1_base32': extract['source_response_sha1_base32'],
           'body_path': str(body), 'headers_path': str(headers), 'stderr_path': str(stderr),
           'error': response.stderr.decode('utf-8', 'replace'), 'exact': False}
    for kind, path in [('headers', headers), ('stderr', stderr)]:
        if path.exists():
            raw = path.read_bytes()
            row[kind + '_bytes'] = len(raw)
            row[kind + '_sha256'] = hashlib.sha256(raw).hexdigest()
    if body.exists():
        raw = body.read_bytes()
        row.update(bytes=len(raw), sha256=hashlib.sha256(raw).hexdigest(),
                   sha1_base32=base64.b32encode(hashlib.sha1(raw).digest()).decode())
        row['exact'] = response.returncode == 0 and row['http_status'] == '200' and row['bytes'] == row['expected_bytes'] and row['sha256'] == row['expected_sha256']
        row['matches_declared_cdx_digest'] = row['sha1_base32'] == row['declared_cdx_sha1_base32']
    journal['attempts'].append(row)
    print(json.dumps({k: row.get(k) for k in ('source_id', 'http_status', 'exit_code', 'bytes', 'exact')}), flush=True)
    if row['http_status'] == '429':
        journal['not_attempted'] = [s['id'] for s in sources[index + 1:]]
        journal['stopped_reason'] = 'HTTP429; respected rate limit, no further requests.'
        save()
        break
    save()
print('EXACT', journal['exact_source_count'], '/24', flush=True)
