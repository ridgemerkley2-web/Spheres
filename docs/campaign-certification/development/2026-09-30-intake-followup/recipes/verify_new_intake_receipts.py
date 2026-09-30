import hashlib
import json
from pathlib import Path

ROOT = Path(r'C:\Users\ridge\Documents\Codex\2026-09-05\pick-up-the-spheres-game-on\work\campaign-certification\integration')
OUT = Path(r'D:\spheres-offload\codex-next-20260928\combined-intakes-34-37-validation')

def verify(raw, row):
    assert len(raw) == row['bytes'] and hashlib.sha256(raw).hexdigest() == row['sha256'], row['path'] if 'path' in row else row

results = []
for number in ('34', '35', '36'):
    folder = ROOT / f'docs/campaign-certification/C01/integrations/CLAUDE-C01-{number}'
    manifest = json.loads((folder / 'manifest.json').read_bytes())
    key = 'files' if 'files' in manifest else 'evidence'
    for row in manifest[key]:
        path = (folder / row['path']).resolve()
        assert path.is_relative_to(folder.resolve())
        verify(path.read_bytes(), row)
    actual = {p.relative_to(folder).as_posix() for p in folder.rglob('*') if p.is_file() and p != folder / 'manifest.json'}
    assert {r['path'] for r in manifest[key]} == actual
    original = json.loads((folder / 'original-review-manifest.json').read_bytes())
    for row in original[key]:
        name = 'original-review-README.md' if row['path'] == 'README.md' else row['path']
        verify((folder / name).read_bytes(), row)
    external = 0
    if number == '35':
        for row in json.loads((folder / 'retrieval.json').read_bytes())['sources']:
            raw = Path(row['body_path']).read_bytes()
            verify(raw, row)
            assert len(raw) == row['expected_bytes'] and hashlib.sha256(raw).hexdigest() == row['expected_sha256']
            external += 1
        for row in json.loads((folder / 'source-verification.json').read_bytes())['sources']:
            extract = row['reviewed_extract']
            # The record includes kind metadata beyond the byte identity.
            verify((ROOT / extract['path']).read_bytes(), extract)
    results.append({'packet': number, 'payloads_verified': len(actual), 'original_review_payloads_reverified': len(original[key]),
                    'additional_external_bodies_rehashed': external, 'new_content_review': False})
text = json.dumps(results, indent=2) + '\n'
(OUT / 'root-receipt-reverification-34-36.json').write_text(text, encoding='utf-8')
print(text)
