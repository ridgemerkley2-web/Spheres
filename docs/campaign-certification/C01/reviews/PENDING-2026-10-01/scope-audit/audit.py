"""Offline scope audit. Run with Python from any directory; makes no network calls.

Only writes scope-audit.json and newtip-triage.json beside this script.
Historical content decisions remain those of the cited independent receipts.
"""
import copy
import hashlib
import json
from pathlib import Path
import re
import subprocess
from collections import Counter
from urllib.parse import unquote

OUT = Path(__file__).resolve().parent
ROOT = next(p for p in OUT.parents if (p / '.git').exists())
BASE = '509bd2890f71850c97215d304ff16b757c7d49c2'
OLD = '9cb02c20012a6e6314d7e64d52241609d566f738'
NEW = 'b66f8c074431d7e1a8c9bfca228576c7d5134655'
PREFIX = 'docs/campaign-certification/C01/research/'

def git(*args):
    return subprocess.check_output(['git', *args], cwd=ROOT, stderr=subprocess.DEVNULL).decode('utf-8').strip()

def stored(rev, path):
    return json.loads(git('show', f'{rev}:{path}'))

def read(path):
    return json.loads((ROOT / path).read_text(encoding='utf-8'))

def digest(obj):
    return hashlib.sha256(json.dumps(obj, sort_keys=True, ensure_ascii=True).encode()).hexdigest()

def write(name, obj):
    (OUT / name).write_text(json.dumps(obj, indent=2, ensure_ascii=False) + '\n', encoding='utf-8')

def role(data, role_id):
    return next(r for o in data['organizations'] for r in o['roles'] if r['id'] == role_id)

def tree(rev):
    rows = git('ls-tree', '-r', rev, '--', PREFIX).splitlines()
    return {r.split('\t', 1)[1]: r.split()[2] for r in rows}

def blob(path):
    return git('hash-object', '--path=' + path, path)

errors = []
def check(condition, detail):
    if not condition:
        errors.append(detail)

# DA follow-up is a new, unaccepted submission; preserve the original held receipt.
da_path = PREFIX + 'south-africa.json'
base_da, old, new = [stored(r, da_path) for r in (BASE, OLD, NEW)]
prior_ids = {s['id'] for s in base_da['sources']}
old_sources = {s['id']: s for s in old['sources'] if s['id'] not in prior_ids}
new_sources = {s['id']: s for s in new['sources'] if s['id'] not in prior_ids}
original_keys = ['source_id', 'source_url', 'original_url', 'archive_capture_utc',
                 'published_date', 'source_response_bytes', 'source_response_sha256',
                 'source_response_sha1_base32', 'source_response_content_encoding']
pins = []
for source_id in sorted(old_sources):
    a, b = old_sources[source_id], new_sources[source_id]
    ea, eb = stored(OLD, a['snapshot']['path']), stored(NEW, b['snapshot']['path'])
    pa, pb = [{k: e.get(k) for k in original_keys} for e in (ea, eb)]
    pins.append({'source_id': source_id, 'original_pins_unchanged': pa == pb,
                 'url_unchanged': a['url'] == b['url'], 'original': pb})
old_claims = {c['id']: c for s in old_sources.values() for c in s['claims']}
new_claims = {c['id']: c for s in new_sources.values() for c in s['claims']}
checked_id = 'za_da_fedex_informed_maimane_resignation_decision_20191024'
material_keys = ('id', 'text', 'attested_on', 'locator')
checked_material_unchanged = all(old_claims[checked_id].get(k) == new_claims[checked_id].get(k) for k in material_keys)
baseline_holders = role(base_da, 'za_da_federal_leader')['holder_claims']
new_holders = role(new, 'za_da_federal_leader')['holder_claims']
added_holders = [h for h in new_holders if h not in baseline_holders]
prior_sources_unchanged = all(s in new['sources'] for s in base_da['sources'])
check(len(new_sources) == 24 and set(old_sources) == set(new_sources), 'DA source identity set changed')
check(len(new_claims) == 29 and set(old_claims) == set(new_claims), 'DA claim identity set changed')
check(all(p['original_pins_unchanged'] and p['url_unchanged'] for p in pins), 'DA original response pins changed')
check(prior_sources_unchanged and all(h in new_holders for h in baseline_holders), 'DA prior accepted sources or holders changed')
check(len(added_holders) == 12 and checked_material_unchanged, 'DA follow-up count or checked material changed')
triage = {
    'decision': 'held; offline diff triage only; no new historical acceptance',
    'prior_review_submission': OLD, 'latest_submission': NEW,
    'prior_review_receipt': 'd1818a48a1b117d408dafb49d7943d35f3f7c2da',
    'changed_paths': git('diff', '--name-only', OLD, NEW).splitlines(),
    'network_requests_this_triage': 0,
    'same_originals': len(pins), 'same_claim_ids': len(new_claims),
    'original_identity_pins': pins,
    'prior_accepted_sources_unchanged': len(base_da['sources']) if prior_sources_unchanged else False,
    'prior_accepted_holder_objects_unchanged': all(h in new_holders for h in baseline_holders),
    'retained_source_checkpoint': {'exact_reproduced': 1, 'http_429': 1, 'not_attempted': 22, 'unverified': 23},
    'claim_checkpoint': {'material_claim_checked': checked_id, 'material_unchanged': checked_material_unchanged,
                         'changed_interpretive_uncertainty_not_approved': True, 'checked': 1, 'held': 28},
    'new_holder_observations': len(added_holders), 'new_holder_observations_held': len(added_holders),
    'holder_dependency_decisions': [
        {'name': h['name'], 'attested_on': h['attested_on'], 'from': h['from'], 'until': h['until'],
         'sources': h['sources'], 'decision': 'held; relevant original content not independently reviewed'}
        for h in added_holders],
    'changes_assessed': [
        'Zille 2007-05-06 acceptance adds an observation without a start; its original was unattempted. A plausible interpretation is not independent approval.',
        'Maimane until 2019-10-23 resolves the proposed Friday 25 October vacancy statement weekday. The Oct25 original returned HTTP429 and remains unread; the read Oct24 statement alone does not independently accept that claimed vacancy boundary.',
        'The 2014 quoted title apostrophe is repaired in the derived extract and claim; its original remains unreviewed.',
        'New test assertions encode the submission interpretation, not independent source evidence. Assertions of user rulings are not approval evidence.'
    ],
    'active_data_import_allowed': False,
    'original_held_receipt_untouched': True,
    'limits': 'No historical-source fetch or follow-up tests; preserves old receipt scoped to 9cb02c20. Unattempted originals are not failed requests.'
}
write('newtip-triage.json', triage)

# Git blob comparisons cover all existing research, including held countries.
baseline_tree, current_tree = tree(BASE), tree('HEAD')
for p in git('diff', 'HEAD', '--name-only', '--', PREFIX).splitlines():
    current_tree[p] = blob(p) if (ROOT / p).exists() else None
for p in git('ls-files', '--others', '--exclude-standard', '--', PREFIX).splitlines():
    current_tree[p] = blob(p)
changed_existing = [p for p, b in baseline_tree.items() if current_tree.get(p) != b]
new_research = sorted(set(current_tree) - set(baseline_tree))
check(changed_existing == [PREFIX + 'japan.json'], 'Existing research change outside Japan')
accepted = read('docs/campaign-certification/C01/reviews/CLAUDE-C01-31-resumed-20261001/scope-audit.json')
accepted_paths = {x['path']: x['git_blob'] for x in accepted['authored_reviewed_paths'] if x['path'].startswith(PREFIX)}
check(all(current_tree.get(p) == b for p, b in accepted_paths.items()), 'Japan differs from independently accepted reviewed blobs')
check(set(new_research) == set(accepted_paths) - {PREFIX + 'japan.json'}, 'Unexpected new research paths')
country_comparisons = [{'path': p, 'base_blob': b, 'current_blob': current_tree.get(p), 'unchanged': b == current_tree.get(p)}
                       for p, b in baseline_tree.items() if p.count('/') == PREFIX.count('/') and p.endswith('.json') and p != PREFIX + 'japan.json']
source_comparisons = [(p, b, current_tree.get(p)) for p, b in baseline_tree.items() if p.startswith(PREFIX + 'sources/')]
before, after = stored(BASE, PREFIX + 'japan.json'), read(PREFIX + 'japan.json')
old_ids = {s['id'] for s in before['sources']}
added_sources = [s for s in after['sources'] if s['id'] not in old_ids]
restored = copy.deepcopy(after)
restored['sources'] = [s for s in restored['sources'] if s['id'] in old_ids]
restored['coverage']['unresolved'] = restored['coverage']['unresolved'][:len(before['coverage']['unresolved'])]
for o in restored['organizations']:
    if o['id'] == 'jp_sangiin_pr_2025_15':
        original = next(x for x in before['organizations'] if x['id'] == o['id'])
        o['roles'] = [r for r in o['roles'] if r['id'] != 'jp_komeito_representative']
        for key in ('sources', 'claim_ids'):
            o[key] = o[key][:len(original[key])]
        o['coverage']['unresolved'] = o['coverage']['unresolved'][:len(original['coverage']['unresolved'])]
check(restored == before, 'Japan prior packet not restored exactly by removing declared appends')
holders = role(after, 'jp_komeito_representative')['holder_claims']
counts = {'prior_sources': len(before['sources']), 'prior_claims': sum(len(s['claims']) for s in before['sources']),
          'new_sources': len(added_sources), 'new_claims': sum(len(s['claims']) for s in added_sources), 'new_holders': len(holders)}
check(list(counts.values()) == [472, 799, 31, 64, 15], 'Japan accepted counts mismatch')

# No runtime, art, or held-country research/test import.
changed = sorted(set(git('diff', BASE, '--name-only').splitlines()) |
                 set(git('ls-files', '--others', '--exclude-standard').splitlines()))
allowed_tools = {'certified_gap_ledger.py', 'test_certified_boundary_matrix.py', 'test_certified_gap_ledger.py',
                 'test_japan_komeito_representatives_c01_31.py', 'test_japan_ldp_presidents_c01_18.py',
                 'test_japan_prime_ministers_c01_12.py', 'test_japan_prime_ministers_c01_13.py',
                 'test_japan_research_s10d.py', 'test_japan_sdp_chairs_c01_29.py',
                 'test_ussr_democratic_russia_soyuz_c01_35.py', 'test_ussr_prior_holder_guard.py', 'test_ussr_research_s10h.py'}
unexpected = [p for p in changed if not p.startswith('docs/') and not (p.startswith('tools/avatars/') and Path(p).name in allowed_tools)]
check(not unexpected, 'Unexpected runtime/assets/code changes: ' + str(unexpected))

queue = read('docs/planning/ai-task-queue.json')['tasks']
claude = [t for t in queue if t['owner'] == 'Claude']
states = dict(Counter(t['state'] for t in claude))
check(states == {'complete': 26, 'ready_for_review': 2}, 'Claude queue counts mismatch')
held = sorted(t['id'] for t in claude if t['state'] != 'complete')
check(held == ['CLAUDE-C01-28', 'CLAUDE-C01-39'], 'Unexpected held Claude tasks')
pins_resolved = []
for t in claude:
    if 'review_revision' in t:
        resolved = git('rev-parse', '--verify', t['review_revision'] + '^{commit}')
        pins_resolved.append({'task': t['id'], 'pin': t['review_revision'], 'commit': resolved})
    check((ROOT / t['handoff']).exists(), 'Missing queue handoff: ' + t['handoff'])
da_task = next(t for t in claude if t['id'] == 'CLAUDE-C01-39')
check(da_task['submission_revision'] == NEW and da_task['review_revision'].startswith('d1818a48') and '9cb02c20' in da_task['next'], 'DA latest submission and original review scopes conflated')

docs = ['CLAUDE.md', 'docs/AI_WORKSTREAMS.md', 'docs/planning/ai-handoffs/CLAUDE-C01-NEXT.md',
        'docs/planning/ai-handoffs/CLAUDE-CAMPAIGN-LEADER-ART.md', 'docs/planning/ai-handoffs/CLAUDE-C01-39.md',
        'docs/campaign-certification/C01/integrations/CLAUDE-C01-31/README.md',
        'docs/campaign-certification/C01/reviews/PENDING-2026-10-01/README.md']
links = []
doc_pins = []
for p in docs:
    data = (ROOT / p).read_bytes()
    doc_pins.append({'path': p, 'sha256': hashlib.sha256(data).hexdigest()})
    for target in re.findall(r'\[[^\]]*\]\(([^)]+)\)', data.decode('utf-8')):
        target = target.strip('<>').split('#')[0]
        if not target or re.match(r'^[a-zA-Z]+:', target):
            continue
        resolved = (ROOT / p).parent / unquote(target)
        exists = resolved.exists()
        links.append({'source': p, 'target': target, 'exists': exists})
        check(exists, 'Broken local link: ' + p + ' -> ' + target)
art = (ROOT / docs[3]).read_text(encoding='utf-8')
check(all(s in art for s in ('1990–2035', '7 September 2026', 'fictional', 'margaret-thatcher-cartoon-1990-v3.png', 'fixed 2D')), 'Current art direction missing an expected boundary')
result = {
    'decision': 'pass' if not errors else 'issues', 'issues': errors,
    'base': BASE, 'head': git('rev-parse', 'HEAD'),
    'method': 'Offline effective-working-tree Git blob comparisons, structured append removal, queue commit resolution and local-link audit. No native builds or network access.',
    'japan_counts': counts, 'japan_prior_packet_restored_exactly': restored == before,
    'accepted_japan_blobs_match': all(current_tree.get(p) == b for p, b in accepted_paths.items()),
    'accepted_japan_research_path_count': len(accepted_paths),
    'other_country_git_blob_comparisons': country_comparisons,
    'prior_source_extracts': {'count': len(source_comparisons), 'comparison_sha256': digest(source_comparisons),
                             'changed': [p for p, a, b in source_comparisons if a != b]},
    'held_country_data_imported': False if all(x['unchanged'] for x in country_comparisons) else 'inspect issues',
    'new_research_paths': new_research,
    'unexpected_non_documentation_paths': unexpected,
    'allowed_tool_changes': [p for p in changed if p.startswith('tools/')],
    'runtime_or_asset_changes': unexpected,
    'claude_queue_states': states, 'held_tasks': held, 'queue_review_commit_pins': pins_resolved,
    'current_document_pins': doc_pins, 'local_links': {'checked': len(links), 'broken': [x for x in links if not x['exists']]},
    'manual_semantic_review': 'Current workboard and three central guides distinguish accepted Komeito bounded research from held Russia/DA, retain old DA receipt scope and latest twelve-holder follow-up, and preserve actual dated campaign identity, fixed cartoon style, historical cutoff and fictional-successor limits. No runtime, portrait or parent qualification is claimed.',
    'new_da_followup_triage': 'newtip-triage.json',
    'limits': 'Source content acceptance belongs to the immutable independent receipts; this audit proves integration scope and publication consistency only. Documentation hashes pin this working-tree checkpoint; later edits require rerun.'
}
write('scope-audit.json', result)
print(json.dumps({'decision': result['decision'], 'issues': errors, 'japan': counts, 'claude': states, 'links': len(links), 'prior_source_extracts': len(source_comparisons)}))
raise SystemExit(bool(errors))
