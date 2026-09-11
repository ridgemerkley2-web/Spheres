from pathlib import Path
import hashlib,json,re,shutil,subprocess,datetime
base=Path(__file__).resolve().parent;repo=base/'integration';ev=base/'evidence'
candidate='db9d17c8d726aa102aa143ceb3599009c558ffee'
read=lambda p:p.read_text(encoding='utf-8-sig')
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=repo,text=True).strip()==candidate
assert not subprocess.check_output(['git','status','--porcelain'],cwd=repo)
native=json.loads(read(ev/'S05-final-native-db9d17c.json'))
for label in ['native','node','binary','browser']:
    value=json.loads(read(ev/f'S05-final-{label}-db9d17c.json'))
    assert value['exit_code']==0 and value['revision_before']==value['revision_after']==candidate and value['clean_before'] and value['clean_after']
counts=re.findall(r'test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored;',read(ev/'S05-final-native-db9d17c.log'))
totals=[sum(int(r[i]) for r in counts) for i in range(3)]
assert totals==[1480,0,75],totals
browser=json.loads(read(ev/'S05-browser-db9d17c/result.json'))
assert browser['passed'] and browser['build_evidence']['revision']==candidate
assert len(browser['build_evidence']['assets'])==20
assert browser['build_evidence']['binary_sha256']==json.loads(read(ev/'S05-final-binary-db9d17c.json'))['binary_sha256']
ext=json.loads(read(ev/'S05-external-runtime-final/result.json'))
assert ext['candidate']==ext['candidate_after']==candidate and ext['clean_after'] and ext['binary_unchanged'] and all(x['exit_code']==0 for x in ext['checks'])
metrics=[]
for name in ['S05-lifetime-confirmation-1','S05-lifetime-confirmation-2']:
    profile=json.loads(read(ev/name/'profile.json'))
    runner=json.loads(read(ev/name/'runner-result.json'))
    assert runner['passed'] and runner['candidate_revision']==candidate and runner['test_binary_sha256']==native['binary_sha256']
    assert runner['memory']['max_sampled_private_bytes']<=1024**3 and len(profile['results'])==6
    for row in profile['results']:
        sim=row['simulation_and_history_recording'];whole=row['whole_server_turn']
        assert sim['samples']==whole['samples']==31 and sim['p95_ms']<=300 and whole['p95_ms']<=400 and whole['max_ms']<=750,row
        metrics.append({'run':name,'scenario':row['scenario'],'checkpoint_years':row['checkpoint_years'],'simulation_p95_ms':sim['p95_ms'],'whole_p95_ms':whole['p95_ms'],'whole_max_ms':whole['max_ms']})
assert len(metrics)==12
linux=json.loads(read(ev/'S05-linux-runtime-final/summary.json'))
assert linux['candidate']==candidate and linux['source_clean_before_and_after'] and linux['fixtures_unchanged']
assert linux['native_full_workspace']['passed']==1480 and linux['native_full_workspace']['failed']==0 and linux['native_full_workspace']['ignored']==75
assert linux['node']=={'tests':1473,'pass':1472,'fail':0,'skipped':1}
assert len(linux['external_checks'])==3 and all(x['exit_code']==0 for x in linux['external_checks'])
preservation=json.loads(read(ev/'S05-preservation-runtime-final.json'))
assert preservation['passed'] and preservation['candidate']==preservation['candidate_after']==candidate and preservation['candidate_clean_after']
comparison=json.loads(read(ev/'S05-deployment-outcome-optimized/result.json'))
assert comparison['passed'] and comparison['comparison_passed'] and comparison['revision']==candidate and len(comparison['checkpoints'])==5
destinations={s:repo/f'docs/campaign-certification/{s}/evidence' for s in ['S04','S05']}
for dest in destinations.values():
    dest.mkdir(exist_ok=True)
    (dest/'.gitattributes').write_text('* -text whitespace=cr-at-eol,-blank-at-eof\n',encoding='utf-8')
def copy(p,dest):
    dest.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(p,dest)
for p in ev.iterdir():
    if p.is_file() and p.name.startswith(('S04','S05')):
        copy(p,destinations[p.name[:3]]/p.name)
for folder in ev.iterdir():
    if not folder.is_dir() or not folder.name.startswith('S05'):continue
    for p in folder.iterdir():
        if not p.is_file():continue
        if folder.name.startswith('S05-live-map'):
            include=p.name in ['visual-observations.json','launch.json','restart.json'] or p.suffix=='.log'
        elif folder.name.startswith('S05-deployment-outcome'):
            include=p.name in ['result.json','requests.jsonl','server.log','server-stderr.log','server-stdout.log']
        else:
            include=p.suffix not in ['.exe','.bak']
        if include:copy(p,destinations['S05']/folder.name/p.name)
scripts=['run-s05-final.py','run-s05-external-runtime-final.py','s05-installed-browser.cjs','record-s05-manual.py','package-s05-evidence.py']
for name in scripts:copy(base/name,destinations['S05']/'runners'/name)
for name in ['run-deployment-outcome.cjs','README.md']:
    copy(base/'s05-staging/deployment-outcome'/name,destinations['S05']/'runners/deployment-outcome'/name)
copy(base/'s05-staging/audit-preservation-runtime-final.cjs',destinations['S05']/'runners/audit-preservation-runtime-final.cjs')
for name in ['S05_EVIDENCE_MAPPING_AUDIT.md','S05_EVIDENCE_MAPPING_AUDIT.json']:
    p=base/'s05-staging'/name
    if p.exists():copy(p,destinations['S05']/name)
copy(base/'s05-staging/W_MATRIX_ANCHORS.json',destinations['S04']/'W_MATRIX_ANCHORS.initial.json')
copy(base/'s05-staging/closeout/W_MATRIX_ANCHORS.final.json',destinations['S04']/'W_MATRIX_ANCHORS.final.json')
for p in (base/'fixtures/s05-original-active/evidence').iterdir():
    if p.is_file():copy(p,destinations['S05']/'original-active-provenance'/p.name)
for name in ['generation.json','stdout.log','stderr.log','profile.json']:
    copy(base/'fixtures/s05-pinned-master'/name,destinations['S05']/'original-master-provenance'/name)
for session,dest in destinations.items():
    files=[{'path':p.relative_to(dest.parent).as_posix(),'sha256':sha(p),'bytes':p.stat().st_size} for p in sorted(dest.rglob('*')) if p.is_file()]
    manifest={'session':session,'runtime_candidate_revision':candidate,'recorded_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'native_windows':{'passed':1480,'failed':0,'ignored':75},'node_windows':{'passed':1472,'failed':0,'skipped':1},'performance':metrics,'files':files,'note':'Final acceptance and exact platform/A-B scope are in README.md. Evidence preserves intermediate failures; filename final on an intermediate record does not supersede its embedded source revision. Full raw canonical A/B archives and expanded original fixtures remain in the isolated local workspace; their hashes, reproducible runner and compressed original inputs are included.'}
    (dest.parent/'manifest.json').write_text(json.dumps(manifest,indent=2)+'\n',encoding='utf-8')
print(json.dumps({'candidate':candidate,'native':totals,'metrics':metrics,'evidence_files':{s:sum(1 for p in d.rglob('*') if p.is_file()) for s,d in destinations.items()}},indent=2))
