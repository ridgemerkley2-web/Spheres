"""Offline post-integration verification; writes only verification-result.json here."""
import datetime
import hashlib
import json
import pathlib
import subprocess


ROOT = pathlib.Path(__file__).resolve().parents[6]
DEST = pathlib.Path(__file__).resolve().parent
BASELINE = '509bd2890f71850c97215d304ff16b757c7d49c2'
RECEIPT_COMMIT = '6133f79b5b00126d4733453fe42839c1ca7390c4'
RECEIPT_PATH = 'docs/campaign-certification/C01/reviews/CLAUDE-C01-28/2026-10-01-missing-only'
RECEIPT = ROOT / RECEIPT_PATH
COUNTRY_PATH = 'docs/campaign-certification/C01/research/russia.json'
SOURCE_DIR = 'docs/campaign-certification/C01/research/sources'
FAILURES = []


def git(*args):
    return subprocess.check_output(['git', *args], cwd=ROOT)


def identity(raw):
    return {'bytes': len(raw), 'sha256': hashlib.sha256(raw).hexdigest()}


def load(path):
    return json.loads(path.read_text(encoding='utf8'))


def check_file(path, expected, label):
    if not path.is_file():
        FAILURES.append({'file': label, 'error': 'missing'})
        return {'file': label, 'matched': False}
    actual = identity(path.read_bytes())
    matched = actual == expected
    if not matched:
        FAILURES.append({'file': label, 'actual': actual, 'expected': expected})
    return {'file': label, **actual, 'matched': matched}


def summary(rows):
    return {
        'files_checked': len(rows),
        'all_match': all(row['matched'] for row in rows),
        'total_bytes_read': sum(row.get('bytes', 0) for row in rows),
        'ordered_actual_identity_record_sha256': hashlib.sha256(
            json.dumps(rows, sort_keys=True, separators=(',', ':')).encode()
        ).hexdigest(),
    }


manifest_raw = (RECEIPT / 'manifest.json').read_bytes()
manifest = json.loads(manifest_raw)
manifest_original = git('show', RECEIPT_COMMIT + ':' + RECEIPT_PATH + '/manifest.json')
if manifest_raw != manifest_original:
    FAILURES.append({'file': RECEIPT_PATH + '/manifest.json', 'error': 'changed from receipt commit'})
payload_rows = []
for entry in manifest['payloads']:
    path = (RECEIPT / entry['path']).resolve()
    if not path.is_relative_to(RECEIPT.resolve()):
        raise ValueError('Receipt payload escapes its directory')
    payload_rows.append(check_file(path, {'bytes': entry['bytes'], 'sha256': entry['sha256']}, entry['path']))
assert len(payload_rows) == 23

old = load(RECEIPT / 'retained-body-recheck.json')['bodies']
new = load(RECEIPT / 'retrieval.json')['sources']
assert len(old) == 56 and len(new) == 10
external_rows = []
for entry in old:
    external_rows.append(check_file(pathlib.Path(entry['body_path']), {'bytes': entry['bytes'], 'sha256': entry['sha256']}, entry['body_path']))
for entry in new:
    external_rows.append(check_file(pathlib.Path(entry['body_path']), {'bytes': entry['bytes'], 'sha256': entry['sha256']}, entry['body_path']))
    external_rows.append(check_file(pathlib.Path(entry['headers_path']), {'bytes': entry['headers_bytes'], 'sha256': entry['headers_sha256']}, entry['headers_path']))
assert len(external_rows) == 76
assert len({row['file'] for row in external_rows}) == 76

base_country_raw = git('show', BASELINE + ':' + COUNTRY_PATH)
country = json.loads(base_country_raw)
base_sources = {
    path for path in git('ls-tree', '-r', '--name-only', BASELINE, SOURCE_DIR).decode().splitlines()
    if pathlib.PurePosixPath(path).name.startswith('russia-')
}
referenced_sources = {source['snapshot']['path'] for source in country['sources'] if source.get('snapshot')}
source_paths = base_sources | referenced_sources
current_sources = {
    path.relative_to(ROOT).as_posix()
    for path in (ROOT / SOURCE_DIR).glob('russia-*') if path.is_file()
}
if current_sources != base_sources:
    FAILURES.append({'error': 'Russia source file set changed', 'added': sorted(current_sources - base_sources), 'missing': sorted(base_sources - current_sources)})
baseline_rows = [check_file(ROOT / COUNTRY_PATH, identity(base_country_raw), COUNTRY_PATH)]
for path in sorted(source_paths):
    baseline_rows.append(check_file(ROOT / path, identity(git('show', BASELINE + ':' + path)), path))

result = {
    'format': 'spheres-russia-post-integration-verification/v1',
    'checked_utc': datetime.datetime.now(datetime.timezone.utc).isoformat(),
    'reviewer': 'Codex /root/retire_selector_review',
    'integration_checkout': str(ROOT),
    'integration_head_at_check': git('rev-parse', 'HEAD').decode().strip(),
    'baseline': BASELINE,
    'receipt_original_commit': RECEIPT_COMMIT,
    'network_requests': 0,
    'script': identity(pathlib.Path(__file__).read_bytes()),
    'receipt_manifest_unchanged_from_original_commit': manifest_raw == manifest_original,
    'receipt_manifest': identity(manifest_raw),
    'receipt_payloads': summary(payload_rows),
    'external_evidence': {
        **summary(external_rows),
        'previous_original_bodies': 56,
        'new_response_bodies_including_429': 10,
        'new_response_headers': 10,
        'all_unique_files': True,
    },
    'baseline_russia_files': {
        **summary(baseline_rows),
        'country_jsons': 1,
        'russia_named_source_files': len(base_sources),
        'referenced_snapshot_files': len(referenced_sources),
        'source_file_set_unchanged': current_sources == base_sources,
        'country_json_identity': identity(base_country_raw),
    },
    'failures': FAILURES,
    'passed': not FAILURES,
    'scope': 'Post-copy byte verification only. Preserves the Russia packet hold, immutable receipts, raw bodies and failed attempts; no historical-content rereview or active research import.',
}
(DEST / 'verification-result.json').write_text(json.dumps(result, indent=2) + '\n', encoding='utf8')
print(json.dumps(result, indent=2))
raise SystemExit(0 if result['passed'] else 1)
