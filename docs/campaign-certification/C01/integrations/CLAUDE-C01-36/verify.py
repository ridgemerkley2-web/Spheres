"""Read-only verification of this bounded C01-36 review packet."""
import argparse
import hashlib
import json
from pathlib import Path
import subprocess

def digest(data):
    return hashlib.sha256(data).hexdigest()

def check_bytes(data, row):
    assert len(data) == row['bytes'], row
    assert digest(data) == row['sha256'], row

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--repo', type=Path)
    parser.add_argument('--external-originals', action='store_true')
    args = parser.parse_args()
    root = Path(__file__).resolve().parent
    manifest = json.loads((root / 'manifest.json').read_text(encoding='utf8'))
    expected = set()
    for row in manifest['evidence']:
        path = root / row['path']
        assert path.resolve().is_relative_to(root) and path.is_file(), row
        assert row['path'] not in expected, row
        expected.add(row['path'])
        check_bytes(path.read_bytes(), row)
    actual = {p.relative_to(root).as_posix() for p in root.rglob('*') if p.is_file() and p.name != 'manifest.json'}
    assert actual == expected, (actual - expected, expected - actual)
    checked_git = 0
    if args.repo:
        for row in json.loads((root / 'reviewed-file-pins.json').read_text())['files']:
            data = subprocess.check_output(['git', '-C', str(args.repo), 'show', f"{row['revision']}:{row['path']}"])
            check_bytes(data, {'bytes': row['git_blob_bytes'], 'sha256': row['git_blob_sha256']})
            checked_git += 1
    checked_originals = 0
    checked_copies = 0
    if args.external_originals:
        for row in json.loads((root / 'source-verification.json').read_text())['sources']:
            check_bytes(Path(row['body']['path']).read_bytes(), row['body'])
            checked_originals += 1
        for row in json.loads((root / 'copy-ledger.json').read_text())['files']:
            check_bytes(Path(row['original']).read_bytes(), row)
            checked_copies += 1
    print(json.dumps({'passed': True, 'payloads': len(expected), 'git_blobs': checked_git,
                      'external_source_bodies': checked_originals, 'external_copied_evidence': checked_copies,
                      'c01_complete': False, 'qualification': False}))

if __name__ == '__main__':
    main()
