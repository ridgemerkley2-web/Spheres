from pathlib import Path
import hashlib, json, subprocess, re
from datetime import datetime, timezone

base = Path(r'C:\Users\ridge\Documents\Codex\2026-09-05\pick-up-the-spheres-game-on\work\campaign-certification')
repo = base / 'integration'
evidence = base / 'evidence' / 'political-calibration-20260922'

def digest(data): return hashlib.sha256(data).hexdigest()
def read_json(name): return json.loads((evidence/name).read_text(encoding='utf-8-sig'))
def git(*args): return subprocess.check_output(['git', '-C', str(repo), *args])
def hash_file(path):
    h=hashlib.sha256()
    with path.open('rb') as f:
        for block in iter(lambda:f.read(1024*1024), b''): h.update(block)
    return h.hexdigest()

a=read_json('iteration-27-prospective-application.json')
b=read_json('iteration-27-build-receipt.json')
r=read_json('iteration-27-result.json')
rows=read_json(a['source_manifest'])
assert digest((evidence/a['plan']).read_bytes())==a['plan_sha256']
assert digest((evidence/a['source_manifest']).read_bytes())==a['source_manifest_sha256']
assert digest((evidence/'iteration-27-prospective-application.json').read_bytes())==r['application_sha256']
assert len(rows)==1932 and len({x['path'].replace('\\','/') for x in rows})==1932
before_status=git('status','--porcelain=v1').decode('utf-8')
head=git('rev-parse','HEAD').decode().strip()
assert head==a['revision']
mismatches=[]
for row in rows:
    p=(repo/row['path']).resolve()
    assert p.is_relative_to(repo.resolve())
    current={'bytes':p.stat().st_size,'sha256':hash_file(p)}
    if any(current[k]!=row[k] for k in current):
        blob=git('show', head+':'+row['path'].replace('\\','/'))
        checkout_blob = blob.replace(b'\r\n', b'\n').replace(b'\n', b'\r\n')
        assert len(checkout_blob)==row['bytes'] and digest(checkout_blob)==row['sha256']
        mismatches.append({'path':row['path'],'baseline_sha256':row['sha256'], 'current':current,'baseline_git_blob_reconstructed_with_CRLF_matches':True,'git_blob_sha256':digest(blob)})
assert [x['path'].replace('\\','/') for x in mismatches]==['spheres-sim/src/dyads.rs'], mismatches
baseline=(evidence/'iteration-27-government-baseline.rs').read_bytes()
assert digest(baseline)==a['baseline_sha256']==r['restoration']['source_sha256']
assert (repo/'spheres-sim/src/government.rs').read_bytes()==baseline
assert git('show',head+':spheres-sim/src/government.rs')==baseline
old=b'pub const ARMY_CRISIS_CONFIDENCE_WEIGHT: f64 = 0.65;'
new=b'pub const ARMY_CRISIS_CONFIDENCE_WEIGHT: f64 = 1.20;'
assert baseline.count(old)==1
candidate=baseline.replace(old,new,1)
assert digest(candidate)==a['candidate_sha256']==b['candidate_source_sha256']
patch=(evidence/'iteration-27-candidate.patch').read_text(encoding='utf-8')
assert [line for line in patch.splitlines() if line.startswith('-') and not line.startswith('---')]==['-'+old.decode()]
assert [line for line in patch.splitlines() if line.startswith('+') and not line.startswith('+++')]==['+'+new.decode()]
assert r['decision']=='REJECTED' and not r['criteria_changed'] and not r['fixtures_changed']
pattern=r'test result: (\w+)\. (\d+) passed; (\d+) failed; (\d+) ignored; \d+ measured; (\d+) filtered out;'
logs={}
for key in ('native','political'):
    item=r[key]; raw=(evidence/item['log']).read_bytes()
    assert digest(raw)==item['sha256']
    text=raw.decode('utf-8-sig')
    summaries=[list(x) for x in re.findall(pattern,text)]
    assert summaries==item.get('summaries',item.get('summary'))
    actual_failures=re.findall(r'^test (\S+) \.\.\. FAILED$',text,re.M)
    # splitlines normalizes original CRLF for anchored status parsing only; raw archive is unchanged.
    actual_failures=[line.split(' ')[1] for line in text.splitlines() if line.startswith('test ') and line.endswith(' ... FAILED')]
    assert sorted(actual_failures)==sorted(item['failures'])
    assert item['exit_code']==101
    logs[key]={'file':item['log'],'sha256':digest(raw),'bytes':len(raw),'summaries':summaries,'failures':actual_failures}
political=(evidence/r['political']['log']).read_text(encoding='utf-8')
assert all(line in political for line in r['political']['metrics'])
assert 'three nations hold 0.50 of a seed\'s coups' in political
assert 'Islamist takeover by 2000 in 6/12 seeds' in political
criteria=(repo/'spheres-sim/tests/bloc_census.rs').read_text(encoding='utf-8')
for exact in ('assert!((4.0..=14.0).contains(&m)', 'assert!(top3 < 0.5', 'assert!(seeds > 6 && seeds <= 10'):
    assert exact in criteria
assert r['political']['command'][1:]==['--include-ignored','--skip','bloc_census','--nocapture','--test-threads=4']
assert not r['development_cohort_run'] and not r['independent_holdout_run']
assert b['source_files_verified']==len(rows) and r['restoration']['all_native_inputs_restored']==len(rows)
assert r['unchanged_source_files_verified']==len(rows)-1
post_application=read_json('empty-stall-optimization/application.json')
assert datetime.fromisoformat(r['restoration']['utc']) < datetime.fromisoformat(post_application.get('applied_utc',post_application.get('utc')))
source_hashes_before={x['path']:hash_file(repo/x['path']) for x in rows}
receipt={
 'schema':'spheres-rejected-candidate-independent-verification/v1',
 'recorded_utc':datetime.now(timezone.utc).isoformat(), 'reviewer':'verify_ui_browser',
 'candidate':'27 confidence1.20 on corrected26', 'decision':'REJECTED; independently consistent with unmodified raw test results',
 'revision':head, 'runtime_baseline':a['runtime_baseline'],
 'source_review':{'listed_inputs':len(rows),'current_matches_baseline':len(rows)-len(mismatches),
   'current_differences_after_recorded_restoration':mismatches,
   'only_candidate_edit':'ARMY_CRISIS_CONFIDENCE_WEIGHT: 0.65 -> 1.20',
   'baseline_government_sha256':digest(baseline),'reconstructed_candidate_sha256':digest(candidate),
   'candidate_reconstructed_by_exact_single_replacement':True,
   'full_baseline_body_omitted_from_archive':'Recoverable from revision Git blob; baseline and candidate SHA verified.',
   'recorded_restoration':'Original result records all1932 native input hashes restored and clean Git status at2026-09-22T23:14:05.947771+00:00.',
   'current_state_scope':'The independent audit ran after the separate authorized empty-stall optimization in dyads.rs. Current government and1931 inputs match baseline; current dyads differs but HEAD LF blob reconstructed with the existing CRLF checkout convention matches its baseline raw hash. This is not a claim that the current worktree is clean.',
   'later_application_receipt':{'path':'empty-stall-optimization/application.json','sha256':hash_file(evidence/'empty-stall-optimization/application.json')},
   'git_status_before_archive':before_status},
 'log_verification':logs,
 'outcome':{'A1':{'median_electoral_coups':12,'required_median_range':[4,14], 'median_top3_share':0.5,'required_top3_comparison':'strictly <0.5','result':'FAIL'},
   'A2':{'seeds_with_islamist_takeover_by_2000':6,'total_seeds':12,'required_count':'greater than6 and at most10','result':'FAIL'},
   'original_suite':'9passed/2failed includes eight other A1-A10 gates plus attribution regression; not eleven independent outcome targets.',
   'correctness':'Sim library1004passed/1failed/25ignored/1filtered; ordinary bloc integration5passed/0failed/7ignored. Ignored/filtered tests are explicitly not passes.'},
 'preserved_rules':{'thresholds_and_cohorts_unchanged':True,'test_or_fixture_edits_in_candidate_patch':False},
 'execution_limits':{'new_Cargo_or_simulations_by_reviewer':False,'candidate_binaries_executed_by_reviewer':False,
    'candidate_binary_hashes':'Recorded by parent build receipt; executable paths are mutable and baseline rebuild is underway, so reviewer does not claim independently rehashed retained candidate binaries.',
    'N200_and_holdout':'Not run according to result and bounded commands. This audit verifies retained evidence and does not independently audit every historical host process.'},
 'checked_evidence':[{ 'path':name,'bytes':(evidence/name).stat().st_size,'sha256':hash_file(evidence/name)} for name in [a['plan'],'checkpoint26-confidence120-independent-review.json','checkpoint26-confidence120-independent-review.md','iteration-27-prospective-application.json','iteration-27-build-receipt.json','iteration-27-result.json',a['source_manifest'],'iteration-27-candidate.patch',r['native']['log'],r['political']['log']]]
}
out=evidence/'iteration-27-independent-verification.json'
assert not out.exists(), 'Do not overwrite a prior independent receipt'
out.write_text(json.dumps(receipt,indent=2)+'\n',encoding='utf-8',newline='\n')
print(json.dumps({'receipt':str(out),'sha256':hash_file(out),'listed_inputs':len(rows),'current_baseline_matches':len(rows)-len(mismatches),'later_delta':mismatches,'result':'REJECTED VERIFIED'},indent=2))