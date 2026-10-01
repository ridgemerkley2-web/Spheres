"""Offline integrity/preservation checks; never re-fetches sources or substitutes for their content review."""
import argparse
import copy
import hashlib
import json
from pathlib import Path
import subprocess

HERE=Path(__file__).resolve().parent
ROOT=HERE.parents[4]

def read(name): return json.loads((HERE/name).read_text(encoding='utf8'))
def blob(revision,path): return subprocess.check_output(['git','show',revision+':'+path],cwd=ROOT)
def digest(path):
    with path.open('rb') as f: return hashlib.file_digest(f,'sha256').hexdigest()

def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--verify-external',action='store_true')
    args=parser.parse_args()
    manifest=read('manifest.json')
    seen=set()
    for item in manifest['payloads']:
        p=Path(item['path'])
        assert not p.is_absolute() and '..' not in p.parts and p.as_posix() not in seen
        seen.add(p.as_posix()); target=HERE/p
        assert target.stat().st_size==item['bytes'] and digest(target)==item['sha256'],str(p)
    decision=read('decision.json'); sources=read('source-review.json'); claims=read('claim-review.json')
    assert decision['decision']=='accepted_bounded_intake' and decision['accepted_for_integration']
    assert not any(decision[k] for k in ('runtime_roster_modified','art_authorized','qualification','author_reported_user_rulings_are_authority'))
    assert len(sources['rows'])==5 and len(claims['rows'])==6 and claims['new_holders']==0
    assert all(r['identity_matches'] and r['materially_read'] and r['accepted_for_integration'] for r in sources['rows'])
    assert all(r['materially_read'] and r['accepted_for_integration'] and not r['holder_or_boundary_promoted'] for r in claims['rows'])
    attempt=read('retrieval-attempt.json')
    assert attempt['only_one_request'] and attempt['status']=='200' and attempt['exit_code']==0
    assert attempt['bytes']==74793 and attempt['sha256']=='082584197664175c38d1003b03384bbc6bb7ade06ba46f2cd6f7cbf3e2998d2d'
    assert attempt['old_held_source_record_unchanged'] and attempt['retry_after']==[]
    scope=read('scope-review.json'); path='docs/campaign-certification/C01/research/tonga.json'
    base,tip,old=decision['base'],decision['submission'],decision['previous_submission']
    prior,candidate,previous=(json.loads(blob(rev,path)) for rev in (base,tip,old))
    assert len(prior['sources'])==207 and sum(len(s['claims']) for s in prior['sources'])==319
    assert candidate['sources'][:207]==prior['sources'] and len(candidate['sources'])==212
    assert sum(len(s['claims']) for s in candidate['sources'][207:])==6
    assert {c['id'] for s in candidate['sources'][207:] for c in s['claims']}=={r['claim_id'] for r in claims['rows']}
    for a,b in zip(candidate['sources'][207:],previous['sources'][207:]):
        a,b=copy.deepcopy(a),copy.deepcopy(b); a.pop('snapshot');b.pop('snapshot');assert a==b
    for category in ('organizations','institutions'):
        assert [e['id'] for e in prior[category]]==[e['id'] for e in candidate[category]]
        for a,b in zip(prior[category],candidate[category]):
            assert a['roles']==b['roles']
            for key in set(a)|set(b):
                if a.get(key)==b.get(key): continue
                if key in ('sources','claim_ids'):
                    assert a['id']=='to_dpfi' and b[key][:len(a[key])]==a[key]
                elif key=='coverage':
                    x,y=copy.deepcopy(a[key]),copy.deepcopy(b[key]);first,second=x.pop('unresolved'),y.pop('unresolved')
                    assert x==y and second[:len(first)]==first
                else: raise AssertionError((a['id'],key))
    a,b=copy.deepcopy(prior),copy.deepcopy(candidate)
    for key in ('sources','organizations','institutions'): a.pop(key);b.pop(key)
    first,second=a['coverage'].pop('unresolved'),b['coverage'].pop('unresolved')
    assert a==b and second[:len(first)]==first and len(second)==len(first)+1
    assert len(scope['prior_extracts'])==194
    for p in scope['prior_extracts']:
        a,b=blob(base,p['path']),blob(tip,p['path'])
        assert a==b and len(a)==p['bytes'] and hashlib.sha256(a).hexdigest()==p['sha256']
    assert scope['registered_handoff_preserved'] and not scope['runtime_art_generated_or_central_files_modified']
    assert len(scope['other_country_blobs'])==8
    for p in scope['other_country_blobs']:
        b=blob(base,p['path']);assert len(b)==p['bytes'] and hashlib.sha256(b).hexdigest()==p['sha256']
    handoff='docs/planning/ai-handoffs/CLAUDE-C01-44.md'
    assert (HERE/'submission-handoff.md').read_bytes()==blob(tip,handoff)
    validation=read('validation.json')
    assert validation['tests_failed']==0 and validation['tests_passed']>=142
    assert all(run['exit_code']==0 for run in validation['runs'])
    external=0
    if args.verify_external:
        inventory=read('external-files.json')
        for item in inventory['files']:
            target=Path(inventory['roots'][item['root']])/item['path']
            assert target.stat().st_size==item['bytes'] and digest(target)==item['sha256'],str(target)
            external+=1
    print(json.dumps(dict(status='receipt_integrity_and_preservation_pass',payloads=len(seen),external_files=external,
                         prior_sources=207,prior_claims=319,prior_extracts=194,new_sources=5,new_claims=6,new_holders=0,
                         decision='accepted_bounded_intake',historical_review_is_not_inferred_from_tests=True)))

if __name__=='__main__': main()
