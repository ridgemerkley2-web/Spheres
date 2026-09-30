"""Read-only verification of the additive C01-29 decision packet."""
import argparse
import hashlib
import json
from pathlib import Path
import subprocess

def check(data, size, sha):
    assert len(data) == size
    assert hashlib.sha256(data).hexdigest() == sha

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--repo', type=Path)
    parser.add_argument('--external-originals', action='store_true')
    args = parser.parse_args()
    root = Path(__file__).resolve().parent
    manifest = json.loads((root / 'manifest.json').read_text(encoding='utf8'))
    expected = set()
    for row in manifest['evidence']:
        p = root / row['path']
        assert p.resolve().is_relative_to(root) and p.is_file()
        assert row['path'] not in expected
        expected.add(row['path'])
        check(p.read_bytes(), row['bytes'], row['sha256'])
    actual = {p.relative_to(root).as_posix() for p in root.rglob('*') if p.is_file() and p.name != 'manifest.json'}
    assert actual == expected, (actual - expected, expected - actual)
    source_count = old_count = original_count = copies = 0
    if args.repo:
        for filename in ('reviewed-file-pins.json', 'prior-checkpoint-pins.json'):
            for row in json.loads((root / filename).read_text())['files']:
                data = subprocess.check_output(['git', '-C', str(args.repo), 'show', f"{row['revision']}:{row['path']}"])
                check(data, row['git_blob_bytes'], row['git_blob_sha256'])
                if filename == 'reviewed-file-pins.json':
                    source_count += 1
                else:
                    old_count += 1
    if args.external_originals:
        for row in json.loads((root / 'source-verification.json').read_text())['sources']:
            check(Path(row['body']).read_bytes(), row['bytes'], row['sha256'])
            original_count += 1
        for row in json.loads((root / 'copy-ledger.json').read_text())['files']:
            check(Path(row['original']).read_bytes(), row['bytes'], row['sha256'])
            copies += 1
    print(json.dumps({'passed': True, 'payloads': len(expected), 'reviewed_git_blobs': source_count,
                      'old_checkpoint_git_blobs': old_count, 'original_bodies': original_count,
                      'copied_evidence': copies, 'c01_complete': False, 'qualification': False}))

if __name__ == '__main__':
    main()
