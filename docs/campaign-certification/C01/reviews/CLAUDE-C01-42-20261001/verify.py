#!/usr/bin/env python3
"""Offline C01-42 receipt verification; no network or historical rereview."""
import argparse
from datetime import datetime
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
HERE=Path(__file__).resolve().parent

def load(name):
    return json.loads((HERE/name).read_text(encoding='utf-8'))

def inside(root,relative):
    path=(root/relative).resolve()
    assert path.is_relative_to(root.resolve()),relative
    return path

def check(raw,row):
    assert len(raw)==row['bytes'],('bytes',row.get('path'))
    assert hashlib.sha256(raw).hexdigest()==row['sha256'],('sha256',row.get('path'))

def blob(repo,rev,path):
    assert re.fullmatch('[0-9a-f]{40}',rev),rev
    assert not Path(path).is_absolute() and '..' not in Path(path).parts,path
    return subprocess.check_output(['git','show',rev+':'+path],cwd=repo,env=dict(os.environ,GIT_OPTIONAL_LOCKS='0'))

def repository_checks(repo,scope):
    base,rev,imp=scope['integration_base'],scope['corrective_commit'],scope['first_source_import']
    allowed={r['path'] for r in scope['reviewed_files']}
    changed=set(subprocess.check_output(['git','diff','--name-only',base,rev],cwd=repo).decode().splitlines())
    assert changed==allowed,(changed-allowed,allowed-changed)
    corrections=set(subprocess.check_output(['git','diff','--name-only',imp,rev],cwd=repo).decode().splitlines())
    assert corrections==set(scope['corrective_paths'])
    assert corrections=={'tools/avatars/test_japan_jcp_dpfp_leaders_c01_42.py','docs/planning/ai-handoffs/CLAUDE-C01-42.md','docs/campaign-certification/C01/research/japan-jcp-chairs-dpfp-representatives-1990-2026-42.md'}
    for row in scope['reviewed_files']:
        check(blob(repo,rev,row['path']),row)
        assert blob(repo,imp,row['path'])==blob(repo,scope['incoming_tip'],row['path']),row['path']
    path='docs/campaign-certification/C01/research/japan.json'
    old,new=json.loads(blob(repo,base,path)),json.loads(blob(repo,rev,path))
    assert len(old['sources'])==503 and len(new['sources'])==535
    assert old['sources']==new['sources'][:503]
    assert sum(len(s['claims']) for s in old['sources'])==863
    assert sum(len(s['claims']) for s in new['sources'])==913
    assert old['research_cutoff']==new['research_cutoff']=='2026-09-07'
    assert old['institutions']==new['institutions']
    oldorg={o['id']:o for o in old['organizations']};neworg={o['id']:o for o in new['organizations']}
    assert oldorg.keys()==neworg.keys()
    targets={'jp_sangiin_pr_2025_01','jp_sangiin_pr_2025_07'}
    newholders=[]
    for oid,prior in oldorg.items():
        current=neworg[oid]
        if oid not in targets:
            assert prior==current,oid
            continue
        for field in prior:
            if field not in ('roles','sources','claim_ids','coverage'):
                assert prior[field]==current[field],(oid,field)
        assert prior['coverage'].keys()==current['coverage'].keys()
        for key,value in prior['coverage'].items():
            if key=='unresolved':
                assert current['coverage'][key][:-1]==value
                assert 'CLAUDE-C01-42' in current['coverage'][key][-1]
            else:
                assert current['coverage'][key]==value
        for field in ('sources','claim_ids'):
            assert current[field][:len(prior[field])]==prior[field],(oid,field)
        assert [r['id'] for r in prior['roles']]==[r['id'] for r in current['roles']]
        for before,after in zip(prior['roles'],current['roles']):
            for field in ('id','title','kind'):
                assert before[field]==after[field]
            for field in ('sources','claim_ids'):
                assert after[field][:len(before[field])]==before[field]
            assert after['scope_note'].startswith(before['scope_note'])
            assert after['holder_claims'][-len(before['holder_claims']):]==before['holder_claims']
            added=after['holder_claims'][:-len(before['holder_claims'])]
            for holder in added:
                assert holder['from'] is None and holder['until'] is None
                if after['id']=='jp_dpfp_representative':
                    assert holder['attested_on']>='2020-09-15'
                newholders.append((after['id'],holder['name'],holder['attested_on'],tuple(holder['claim_ids'])))
    assert len(newholders)==19
    reviewed=load('holder-review.json')['observations']
    assert newholders==[(h['role_id'],h['name'],h['attested_on'],tuple(h['claim_ids'])) for h in reviewed]
    sources=new['sources'][503:]
    assert [s['id'] for s in sources]==scope['new_source_ids']
    assert [c['id'] for s in sources for c in s['claims']]==scope['new_claim_ids']
    for source in sources:
        raw=blob(repo,rev,source['snapshot']['path']);check(raw,source['snapshot']);assert b'\r' not in raw
        extract=json.loads(raw);assert extract['scope_note']==source['scope_note']
        assert len(source['claims'])==len(extract['rows'])
        for claim,row in zip(source['claims'],extract['rows']):
            assert (claim['id'],claim['text'],claim['locator'],claim.get('attested_on'))==(row['claim_id'],row['text'],row['locator'],row['attested_on'])

def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--repo',type=Path);parser.add_argument('--originals',type=Path);parser.add_argument('--require-accepted',action='store_true');args=parser.parse_args()
    manifest=load('manifest.json');expected={r['path'] for r in manifest['files']}
    assert len(expected)==len(manifest['files'])
    actual={p.relative_to(HERE).as_posix() for p in HERE.rglob('*') if p.is_file() and p.name!='manifest.json' and '__pycache__' not in p.parts}
    assert actual==expected,{'missing':sorted(expected-actual),'extra':sorted(actual-expected)}
    for row in manifest['files']:check(inside(HERE,row['path']).read_bytes(),row)
    assert (HERE/'.gitattributes').read_text().splitlines()==['* -text whitespace=cr-at-eol','*.log -whitespace','*.patch -whitespace']
    sources,claims,holders=load('source-review.json'),load('claim-review.json'),load('holder-review.json')
    assert len(sources['sources'])==32 and sources['sources_held']==0
    assert len(claims['claims'])==50 and claims['claims_held']==0
    assert len(holders['observations'])==19 and holders['holder_observations_held']==0
    cids={c['claim_id'] for c in claims['claims']};assert len(cids)==50
    assert cids=={cid for s in sources['sources'] for cid in s['claim_ids']}
    for holder in holders['observations']:
        assert set(holder['claim_ids'])<=cids and holder['from_date'] is None and holder['until_date'] is None
    retrieval=load('retrieval.json');assert len(retrieval)==32
    previous=None
    for row in retrieval:
        assert row['exit_code']==0 and row['http_status']=='200' and row['matches_submitted']
        assert row['bytes']==row['expected_bytes'] and row['sha256']==row['expected_sha256']
        source=next(s for s in sources['sources'] if s['source_id']==row['source_id'])
        assert (source['sha256'],source['bytes'],source['url'])==(row['sha256'],row['bytes'],row['command'][-1])
        instant=datetime.fromisoformat(row['started_utc'])
        if previous:assert (instant-previous).total_seconds()>=15
        previous=instant
    validation=load('validation/status.json')
    for label,result in validation['observed_results'].items():
        command=load('validation/'+label+'.json');check(inside(HERE/'validation',command['log']).read_bytes(),command)
        assert command['exit_code']==result['exit_code']==(1 if label=='guard-red' else 0),label
    checked=0
    if args.originals:
        for row in sources['external_artifacts']:
            check(inside(args.originals,row['path']).read_bytes(),row);checked+=1
    if args.repo:repository_checks(args.repo,load('scope-review.json'))
    decision=load('decision.json')
    if args.require_accepted:
        assert decision['status']=='accepted_bounded_research' and validation['status']=='passed_focused_checks'
    print(json.dumps(dict(receipt_files_checked=len(expected),external_artifacts_checked=checked,repository_scope_checked=bool(args.repo),decision=decision['status'],historical_content_review_repeated=False,parent_qualification_claimed=False),indent=2))

if __name__=='__main__':main()
