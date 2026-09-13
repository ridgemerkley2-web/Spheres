#!/usr/bin/env python3
"""Run one genuine S08 exporter from a pinned, already-built release test binary.

Usage: python run-s08-supplier-export.py PIN TEST_BINARY SHA256 UNIQUE_LABEL
       [--replay-of PREVIOUS_RUNNER_RESULT]

No build or server launch. Each attempt gets a new evidence directory. A replay
starts another fresh game; it never advances or overwrites an earlier archive.
"""
import argparse
import datetime
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import time

BASE=Path(__file__).resolve().parent
REPO=BASE/'integration'
TEST='equipment_view::company_view_tests::s08_fresh_tonga_paid_supplier_market_export'

def stamp():return datetime.datetime.now(datetime.timezone.utc).isoformat()
def sha(path):
    with Path(path).open('rb') as handle:return hashlib.file_digest(handle,'sha256').hexdigest()
def git(*args):return subprocess.check_output(['git','-c','core.longpaths=true',*args],cwd=REPO,text=True).strip()
def source():return {'captured_utc':stamp(),'revision':git('rev-parse','HEAD'),'status_porcelain':git('status','--porcelain')}
def read(path):return json.loads(Path(path).read_text(encoding='utf-8-sig'))
def write(path,value):
    with Path(path).open('x',encoding='utf-8') as handle:json.dump(value,handle,indent=2);handle.write('\n')
def archived_world(path):
    value=read(path)
    while isinstance(value,dict) and isinstance(value.get('world'),dict):value=value['world']
    assert isinstance(value,dict) and isinstance(value.get('nations'),list),'No native world in '+str(path)
    return value

def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('candidate');parser.add_argument('test_binary',type=Path)
    parser.add_argument('binary_sha256');parser.add_argument('label')
    parser.add_argument('--replay-of',type=Path)
    args=parser.parse_args()
    assert re.fullmatch('[0-9a-f]{40}',args.candidate),'Use a complete candidate revision'
    assert re.fullmatch('[0-9a-f]{64}',args.binary_sha256),'Use a complete lowercase binary SHA256'
    assert re.fullmatch('[A-Za-z0-9_-]+',args.label),'Use a simple unique evidence label'
    binary=args.test_binary.resolve();before=source()
    assert before['revision']==args.candidate and not before['status_porcelain'],'Exporter requires the exact clean candidate'
    assert sha(binary)==args.binary_sha256,'Already-built test binary does not match its expected SHA256'
    expected_short=git('rev-parse','--short=12',args.candidate)
    previous=read(args.replay_of) if args.replay_of else None
    if previous:assert previous.get('candidate')==args.candidate and previous.get('test_binary_sha256_before')==args.binary_sha256,'A declared replay must use the same source and binary'
    out=BASE/'evidence'/('S08-genuine-supplier-'+args.label)
    out.mkdir(exist_ok=False)
    exported=out/'export'
    env=os.environ.copy()
    # Only this explicit ignored test runs. Prevent inherited optional exporters
    # or profile modes from redirecting output into an earlier qualification.
    removed=[]
    for key in list(env):
        if key.startswith('SPHERES_PROFILE') or key.startswith('SPHERES_S08_') or key in ('SPHERES_S02_UI_FIXTURE','SPHERES_S03_UI_FIXTURE','SPHERES_S04_UI_FIXTURE'):
            removed.append(key);env.pop(key)
    env['SPHERES_S08_SUPPLIER_EXPORT']=str(exported)
    command=[str(binary),TEST,'--ignored','--exact','--nocapture','--test-threads=1']
    record={'format':'spheres-s08-genuine-export-qualification','version':1,'candidate':args.candidate,
        'test_binary':str(binary),'test_binary_sha256_before':args.binary_sha256,
        'runner':str(Path(__file__).resolve()),'runner_sha256':sha(__file__),
        'source_before':before,'command':command,'working_directory':str(REPO),
        'export_directory':str(exported),'environment':{'SPHERES_S08_SUPPLIER_EXPORT':str(exported)},
        'sanitized_environment_keys':sorted(removed),'maximum_native_days':3000,
        'started_utc':stamp(),'exit_code':None,'passed':False,'files':[]}
    if previous:record['replay_of']={'path':str(args.replay_of.resolve()),'sha256':sha(args.replay_of),'note':'Fresh replay from ordinary initialization, not mutation of earlier saves.'}
    write(out/'launch.json',record)
    start=time.monotonic()
    try:
        with (out/'stdout-stderr.log').open('xb') as log:
            result=subprocess.run(command,cwd=REPO,env=env,stdout=log,stderr=subprocess.STDOUT,
                creationflags=subprocess.CREATE_NO_WINDOW if os.name=='nt' else 0)
        record['exit_code']=result.returncode
        text=(out/'stdout-stderr.log').read_text(encoding='utf-8',errors='replace')
        totals=[tuple(map(int,m)) for m in re.findall(r'test result: (?:ok|FAILED)\. (\d+) passed; (\d+) failed; (\d+) ignored;',text)]
        record['test_summaries']=totals
        assert result.returncode==0 and totals==[(1,0,0)],'Genuine exporter did not pass exactly its requested ignored test'
        provenance=read(exported/'provenance.json');outcome=read(exported/'result.json')
        record['native_provenance']=provenance;record['native_outcome']=outcome
        assert provenance['source_head']==args.candidate and provenance['source_status']=='','Native source provenance differs'
        assert provenance['build']['revision']==expected_short==outcome['build']['revision'],'Executable embeds a different candidate'
        assert os.path.samefile(provenance['executable'],binary),'Exporter ran a different executable'
        assert outcome['passed'] is True and outcome['no_synthetic_endowments'] is True
        assert 1<=outcome['actual_days_advanced']<=3000
        archives=sorted(exported.glob('*.campaign.json'))
        names={p.name for p in archives}
        required=['fresh-before-decisions','opening','ready-before-purchase','purchased','delivered']
        for label in required:assert any(name.endswith('-'+label+'.campaign.json') for name in names),'Missing genuine '+label+' archive'
        assert '0360-progress.campaign.json' in names,'Missing actual paid-construction preparation archive'
        record['archive_phases']=[{'file':p.name,'sha256':sha(p),'bytes':p.stat().st_size} for p in archives]
        if previous:
            prior_dir=Path(previous['export_directory'])
            old=sorted(prior_dir.glob('*.campaign.json'))
            assert [p.name for p in archives]==[p.name for p in old],'Fresh replay changed checkpoint dates/phases'
            comparisons=[]
            for a,b in zip(archives,old):
                identical=archived_world(a)==archived_world(b)
                comparisons.append({'phase':a.name,'native_world_identical':identical,'previous_sha256':sha(b),'fresh_sha256':sha(a)})
            record['replay_comparison']=comparisons
            assert all(c['native_world_identical'] for c in comparisons),'Fresh same-binary replay changed native world state'
        record['native_validation_passed']=True
    except BaseException as error:
        record['runner_error']=str(error)
    finally:
        record.update(finished_utc=stamp(),wall_seconds=time.monotonic()-start)
        try:
            after=source();record['source_after']=after;record['test_binary_sha256_after']=sha(binary)
            record['integrity_passed']=after['revision']==args.candidate and not after['status_porcelain'] and record['test_binary_sha256_after']==args.binary_sha256
        except BaseException as error:record['integrity_passed']=False;record['integrity_error']=str(error)
        for p in sorted(out.rglob('*')):
            if p.is_file():record['files'].append({'path':p.relative_to(out).as_posix(),'sha256':sha(p),'bytes':p.stat().st_size})
        record['passed']=record.get('native_validation_passed',False) and record['integrity_passed'] and 'runner_error' not in record
        write(out/'runner-result.json',record)
        print(json.dumps({'passed':record['passed'],'evidence':str(out),'exit_code':record['exit_code'],'error':record.get('runner_error')},indent=2),flush=True)
    return 0 if record['passed'] else 1

if __name__=='__main__':raise SystemExit(main())
