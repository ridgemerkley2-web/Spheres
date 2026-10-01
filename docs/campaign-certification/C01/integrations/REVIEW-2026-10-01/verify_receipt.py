"""Read-only combined review receipt checks; not a historical re-review."""
import hashlib
import json
from pathlib import Path

root = Path(__file__).resolve().parent
manifest = json.loads((root / 'manifest.json').read_text(encoding='utf-8'))
seen = set()
for row in manifest['files']:
    path = (root / row['path']).resolve(strict=True)
    assert path.is_relative_to(root) and row['path'] not in seen
    seen.add(row['path'])
    data = path.read_bytes()
    assert len(data) == row['bytes'] and hashlib.sha256(data).hexdigest() == row['sha256'], row['path']
assert seen == {p.relative_to(root).as_posix() for p in root.rglob('*') if p.is_file() and p.name != 'manifest.json'}
scope = json.loads((root / 'scope-audit.json').read_text(encoding='utf-8'))
assert not scope['game_source_data_assets_changed']
assert len(scope['unimported_country_pins']) == 6
assert sum(s['added_sources'] for s in scope['accepted_source_additions']) == 52
assert sum(s['added_claims'] for s in scope['accepted_source_additions']) == 83
final = json.loads((root / 'validation/validation-02/results.json').read_text(encoding='utf-8'))
assert final['passed'] and all(c['exit_code'] == 0 for c in final['commands'])
original = json.loads((root / 'validation/validation-01/results.json').read_text(encoding='utf-8'))
assert not original['passed']
print(json.dumps({'receipt_integrity_passed': True, 'payloads': len(seen), 'final_checks_passed': True,
                  'earlier_failures_retained': True, 'source_content_review_repeated': False,
                  'runtime_mapping_or_parent_qualification': False}))
