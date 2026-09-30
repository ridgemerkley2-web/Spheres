"""Sequential exact-body retrieval for independent C01-36 source review."""
import argparse
import base64
from datetime import datetime, timezone
import hashlib
import json
from pathlib import Path
import subprocess
import time

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--root', type=Path, required=True)
parser.add_argument('--out', type=Path, required=True)
parser.add_argument('--prior', type=Path)
args = parser.parse_args()
args.out.mkdir(parents=True, exist_ok=False)
(args.out / 'bodies').mkdir()
(args.out / 'headers').mkdir()
packet = json.loads((args.root / 'docs/campaign-certification/C01/research/tonga.json').read_text(encoding='utf-8'))
prior = json.loads(subprocess.check_output(['git', '-C', str(args.root), 'show',
    '44098c5a:docs/campaign-certification/C01/research/tonga.json']))
old_ids = {s['id'] for s in prior['sources']}
sources = [s for s in packet['sources'] if s['id'] not in old_ids]
assert len(sources) == 39
completed = set()
if args.prior:
    completed = {r['source_id'] for r in json.loads(args.prior.read_text())['results'] if r['exact_response']}
results = []
for source in sources:
    sid = source['id']
    if sid in completed:
        continue
    extract_path = args.root / source['snapshot']['path']
    extract_bytes = extract_path.read_bytes()
    assert len(extract_bytes) == source['snapshot']['bytes']
    assert hashlib.sha256(extract_bytes).hexdigest() == source['snapshot']['sha256']
    extract = json.loads(extract_bytes)
    body = args.out / 'bodies' / (sid + '.bin')
    headers = args.out / 'headers' / (sid + '.txt')
    command = ['curl.exe', '--silent', '--show-error', '--connect-timeout', '15', '--max-time', '60',
        '-H', 'Accept-Encoding: identity', '--output', str(body), '--dump-header', str(headers),
        '--write-out', '%{json}', source['url']]
    started = datetime.now(timezone.utc).isoformat()
    before = time.monotonic()
    process = subprocess.run(command, capture_output=True)
    elapsed = time.monotonic() - before
    data = body.read_bytes() if body.exists() else b''
    metadata = json.loads(process.stdout.decode('utf-8')) if process.stdout else {}
    digest = hashlib.sha256(data).hexdigest()
    expected = {'bytes': extract['source_response_bytes'], 'sha256': extract['source_response_sha256']}
    result = {'source_id': sid, 'url': source['url'], 'original_url': source['original_url'],
        'started_utc': started, 'finished_utc': datetime.now(timezone.utc).isoformat(),
        'elapsed_seconds': elapsed, 'command': command, 'exit_code': process.returncode,
        'curl': metadata, 'stderr': process.stderr.decode('utf-8', errors='replace'),
        'body': {'path': str(body), 'bytes': len(data), 'sha256': digest,
                 'sha1_base32': base64.b32encode(hashlib.sha1(data).digest()).decode()},
        'headers': str(headers), 'expected_response': expected,
        'extract': {'path': source['snapshot']['path'], 'bytes': len(extract_bytes),
                    'sha256': hashlib.sha256(extract_bytes).hexdigest()},
        'exact_response': process.returncode == 0 and metadata.get('http_code') == 200
            and len(data) == expected['bytes'] and digest == expected['sha256']}
    results.append(result)
    with (args.out / 'attempts.jsonl').open('a', encoding='utf-8', newline='\n') as f:
        f.write(json.dumps(result, ensure_ascii=False) + '\n')
    print(sid, 'EXACT' if result['exact_response'] else 'FAILED', metadata.get('http_code'), len(data), flush=True)
    time.sleep(0.25)
receipt = {'format': 'spheres-independent-source-retrieval/v1', 'task': 'CLAUDE-C01-36',
    'reviewer': 'Codex /root/review_gap_submission', 'reviewed_commit': '6b82475ea8224a7fecf911f6442e8b6d7b506ca1',
    'prior': str(args.prior) if args.prior else None, 'skipped_previously_exact': sorted(completed),
    'results': results, 'exact_responses': sum(r['exact_response'] for r in results),
    'failures': sum(not r['exact_response'] for r in results)}
with (args.out / 'retrieval.json').open('x', encoding='utf-8', newline='\n') as f:
    json.dump(receipt, f, ensure_ascii=False, indent=2)
    f.write('\n')
print(json.dumps({'exact': receipt['exact_responses'], 'failed': receipt['failures']}), flush=True)
