"""Compare scoped C01-39 intake to accepted South Africa data; no source approval."""
import copy
import hashlib
import json
import pathlib
import subprocess

ROOT = pathlib.Path(__file__).resolve().parents[6]
OUT = pathlib.Path(__file__).resolve().parents[1]
BASE = '509bd2890f71850c97215d304ff16b757c7d49c2'
TIP = '9cb02c20012a6e6314d7e64d52241609d566f738'
IMPORT = 'bdb15d2156a020267a989430fbd3a1849d6924db'
PATH = 'docs/campaign-certification/C01/research/south-africa.json'


def blob(ref, path):
    return subprocess.check_output(['git', 'show', ref + ':' + path], cwd=ROOT)


old = json.loads(blob(BASE, PATH))
new = json.loads((ROOT / PATH).read_text(encoding='utf-8'))
assert old == json.loads(blob('d897f024^', PATH))
old_ids = {s['id'] for s in old['sources']}
sources = [s for s in new['sources'] if s['id'] not in old_ids]
assert len(old_ids) == 245 and len(sources) == 24
claims = {c['id'] for s in sources for c in s['claims']}
assert len(claims) == 29
org_id = 'za_iec_n2024_027'
old_org = next(o for o in old['organizations'] if o['id'] == org_id)
new_org = next(o for o in new['organizations'] if o['id'] == org_id)
role_id = 'za_da_federal_leader'
old_role = next(r for r in old_org['roles'] if r['id'] == role_id)
new_role = next(r for r in new_org['roles'] if r['id'] == role_id)
old_holders = old_role['holder_claims']
assert len(old_holders) == 2
assert all(h in new_role['holder_claims'] for h in old_holders)
assert len(new_role['holder_claims']) == 13
added_holders = [h for h in new_role['holder_claims'] if h not in old_holders]
assert len(added_holders) == 11

trimmed = copy.deepcopy(new)
trimmed['sources'] = [s for s in trimmed['sources'] if s['id'] in old_ids]
trim_org = next(o for o in trimmed['organizations'] if o['id'] == org_id)
trim_role = next(r for r in trim_org['roles'] if r['id'] == role_id)
trim_role['holder_claims'] = [h for h in trim_role['holder_claims'] if h in old_holders]
trim_role['scope_note'] = old_role['scope_note']
for obj in [trim_org, trim_role]:
    obj['sources'] = [sid for sid in obj['sources'] if sid in old_ids]
    obj['claim_ids'] = [cid for cid in obj['claim_ids'] if cid not in claims]
trim_org['coverage']['unresolved'] = [u for u in trim_org['coverage']['unresolved'] if 'CLAUDE-C01-39' not in u]
trimmed['coverage']['unresolved'] = [u for u in trimmed['coverage']['unresolved'] if 'CLAUDE-C01-39' not in u]
assert trimmed == old, 'Undeclared preexisting-data modification'
paths = subprocess.check_output(['git', 'diff-tree', '--no-commit-id', '--name-only', '-r', IMPORT], cwd=ROOT, text=True).splitlines()
assert len(paths) == 33
import_files = []
for path in paths:
    raw = blob(IMPORT, path)
    assert raw == blob(TIP, path), path
    import_files.append({'path': path, 'bytes': len(raw), 'sha256': hashlib.sha256(raw).hexdigest()})
result = {
    'base': BASE, 'submission': TIP, 'scoped_import': IMPORT,
    'authored_import': import_files, 'generated_index_imported': False,
    'central_handoff_preserved_normalizing_checkout_line_endings': blob(BASE, 'docs/planning/ai-handoffs/CLAUDE-C01-39.md').decode('utf-8') == (ROOT / 'docs/planning/ai-handoffs/CLAUDE-C01-39.md').read_text(encoding='utf-8'),
    'pre_packet_author_south_africa_equals_accepted_base': True,
    'removing_only_declared_additions_and_scope_note_rewrite_reproduces_base': True,
    'prior_sources_unchanged': 245, 'new_sources': 24, 'new_claims': 29,
    'old_s10h_holder_objects_unchanged': old_holders,
    'submitted_new_holder_observations': added_holders,
    'new_holders': 11, 'all_da_observations': 13,
    'source_content_acceptance': 'Not implied by scope or structural checks; see independent source and claim review.'
}
(OUT / 'scope-audit.json').write_text(json.dumps(result, ensure_ascii=False, indent=2) + '\n', encoding='utf-8', newline='\n')
print('33 exact imported paths; 245 prior sources and both S10h holders unchanged; bounded 24/29/11 additions.')
