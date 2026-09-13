"""Publish a NEW bounded S10.f checkpoint only after its declared proofs pass."""
import datetime,hashlib,importlib.util,json,pathlib,re,subprocess,sys
BASE=pathlib.Path(__file__).resolve().parent
REPO=BASE/'integration'
DEST=REPO/'docs/campaign-certification/S10/f'
PIN='7d85b9f76a3e787b257613442fb0a3370e90f4c2'

def read(p):return json.loads(pathlib.Path(p).read_text(encoding='utf-8-sig'))
def write_text(p,text):
    with pathlib.Path(p).open('w',encoding='utf8',newline='\n') as stream:stream.write(text)
def sha(p):
    with pathlib.Path(p).open('rb') as f:return hashlib.file_digest(f,'sha256').hexdigest()
def git(*args):return subprocess.check_output(['git','-c','core.longpaths=true',*args],cwd=REPO,text=True).strip()
def module(name,file):
    spec=importlib.util.spec_from_file_location(name,BASE/file);m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m);return m
def passed(p):
    r=read(p);assert r.get('passed') is True,str(p);return r
def native_totals(log):
    rows=[tuple(map(int,m)) for m in re.findall(r'test result: (?:ok|FAILED)\. (\d+) passed; (\d+) failed; (\d+) ignored;',log)]
    assert rows,'No native test totals'
    totals={'passed':sum(r[0] for r in rows),'failed':sum(r[1] for r in rows),'ignored':sum(r[2] for r in rows),'completed_targets':len(rows)}
    assert totals['passed']>0 and totals['failed']==0;return totals
def proof(name,lane):
    p=BASE/'evidence'/name;r=passed(p)
    assert r['revision']==r['revision_after']==PIN and r['clean_before'] and r['clean_after'] and r['exit_code']==0
    assert sha(p.with_suffix('.log'))==r['log_sha256']
    assert r['runner_sha256']==sha(BASE/'run-s10f-check.py')
    log=p.with_suffix('.log').read_text(encoding='utf8',errors='replace')
    if lane in ('web','agency','leadership'):assert native_totals(log)==r['totals']
    if lane=='node':
        totals={k:int(re.findall(r'ℹ '+k+r' (\d+)',log)[-1]) for k in ['tests','pass','fail','skipped']}
        assert totals==r['totals'] and totals['pass']>0 and totals['fail']==0 and totals['tests']==totals['pass']+totals['skipped']
    return r
def checked_logs(folder,record,runner):
    assert record['revision']==PIN and record['runner_sha256']==sha(BASE/runner)
    for check in record['checks']:
        log=folder/check['log'];assert check['exit_code']==0 and sha(log)==check['log_sha256']
    return record['checks']
def audit_case(case,fixture=None):
    roots=[case.resolve()]+([fixture.resolve()] if fixture else [])
    records=list((case/'archive-audit').glob('*.json'));assert records
    sources={}
    for p in records:
        r=read(p);assert r['canonical']['ignored_paths']==[]
        for key in ['input','canonical']:
            part=r[key];source=pathlib.Path(part['path']).resolve()
            assert any(source.is_relative_to(root) for root in roots)
            assert source.is_file() and not source.is_symlink()
            assert sha(source)==part['sha256'] and source.stat().st_size==part['bytes']
            sources[str(source)]=(part['sha256'],part['bytes'])
    comparisons=[]
    for line in (case/'progress.jsonl').read_text(encoding='utf8').splitlines():
        r=json.loads(line)
        if r.get('event')!='exact-native-comparison':continue
        assert r['equal'] and r['left']['ignored_paths']==r['right']['ignored_paths']==[]
        assert r['left']['sha256']==r['right']['sha256']
        for side in ('left','right'):
            p=pathlib.Path(r[side]['path']).resolve()
            assert sources[str(p)]==(r[side]['sha256'],r[side]['bytes'])
        assert r['bytes_compared']==r['left']['bytes']==r['right']['bytes']
        assert pathlib.Path(r['left']['path']).read_bytes()==pathlib.Path(r['right']['path']).read_bytes()
        comparisons.append(r)
    assert comparisons
    return {'inspections':len(records),'exact_comparisons':len(comparisons),'ignored_paths':[]}

def main():
    assert len(sys.argv)==2,'Pass the exact committed authored browser driver revision'
    driver_revision=sys.argv[1];assert re.fullmatch('[0-9a-f]{40}',driver_revision)
    final_delta=['tools/ui/ci-succession-context.cjs']
    assert git('rev-parse','HEAD')==driver_revision and not git('status','--porcelain')
    assert git('diff','--name-only',PIN,driver_revision).splitlines()==final_delta
    assert not git('diff','--name-only',PIN,driver_revision,'--','spheres-sim','spheres-cli','spheres-web','Cargo.toml','Cargo.lock')
    assert not DEST.exists(),'Publish before writing S10/f docs; never replace prior evidence'
    proofs={lane:proof('S10-f-'+lane+'.json',lane) for lane in ['web','node','leadership','agency','binary','browser']}
    web,node,leadership,agency,binary=[proofs[k] for k in ['web','node','leadership','agency','binary']]
    assert proofs['browser']['binary']['sha256']==proofs['browser']['binary_sha256_before']==binary['binary']['sha256']
    for key in ['test_binary']:
        assert sha(web[key]['path'])==web[key]['sha256']
    assert sha(binary['binary']['path'])==binary['binary']['sha256']
    linux_dir=BASE/'evidence/S10f-linux-final';linux=passed(linux_dir/'result.json')
    checks=checked_logs(linux_dir,linux,'run-s10f-linux.py');assert len(checks)==3
    assert linux['source_before']=={'head':PIN,'status':''}
    for c in checks:
        assert c['source_after']=={'head':PIN,'status':''}
        assert native_totals((linux_dir/c['log']).read_text(encoding='utf8',errors='replace'))==c['totals']
    assert {c['name'] for c in checks}=={'web','leadership','agency'}
    content_dir=BASE/'evidence/S10f-content-final';content=passed(content_dir/'result.json')
    checks=checked_logs(content_dir,content,'run-s10f-content.py')
    assert content['clean_after'] and content['revision_after']==PIN and len(checks)==11
    expected_tests={'brazil-tests','india-tests','japan-tests','census-tests','france-tests','portrait-tests'}
    test_totals={}
    for c in checks:
        if c['name'] in expected_tests:
            log=(content_dir/c['log']).read_text(encoding='utf8',errors='replace')
            counts=re.findall(r'Ran (\d+) tests? in ',log);assert len(counts)==1 and int(counts[0])>0
            assert re.search(r'^OK\s*$',log,re.M) and not re.search(r'^FAILED\b',log,re.M)
            test_totals[c['name']]=int(counts[0])
    assert set(test_totals)==expected_tests
    authored_dir=BASE/'evidence/S10f-succession-qualified'
    wrapper=passed(authored_dir/'result.json')
    checked_logs(authored_dir,wrapper,'run-s10f-succession.py')
    assert wrapper['clean_after'] and wrapper['driver_revision']==wrapper['driver_revision_after']==driver_revision
    assert wrapper['binary_sha256_after']==binary['binary']['sha256']
    assert len(wrapper['checks'])==1 and wrapper['checks'][0]['name']=='browser','Reuse the unchanged native fixture; do not invent a repeated export'
    fixture=BASE/'evidence/S10f-succession-fixture-2';fixture_manifest=read(fixture/'manifest.json')
    assert fixture_manifest['fixture']=='s10f-authored-native-succession-context'
    assert fixture_manifest['compiled_revision']==PIN[:12] and fixture_manifest['days_advanced']==0
    cases={}
    for label,folder,driver in [('authored',authored_dir/'browser','ci-succession-context.cjs'),('ordinary',BASE/'evidence/S10-browser-f-browser','ci-government-decisions.cjs')]:
        results=list(folder.glob('*/result.json'));assert len(results)==1
        case=results[0].parent;r=passed(results[0])
        assert r['build']['revision']==PIN and r['build']['binary_sha256']==binary['binary']['sha256']
        assert r['test_source']['revision']==(driver_revision if label=='authored' else PIN) and r['test_source']['runtime_source_equal'] and not r['errors']
        assert r['test_source']['driver_sha256']==sha(REPO/'tools/ui'/driver)
        if label=='ordinary':assert r['test_source']['review_assertions_sha256']==sha(REPO/'tools/ui/government-review-assertions.cjs')
        for name,asset in r['build']['assets'].items():
            source=REPO/'spheres-web/ui'/name
            assert source.is_file() and sha(source)==asset['served_sha256']==asset['checkout_sha256']
            committed=subprocess.check_output(['git','-c','core.longpaths=true','show',PIN+':spheres-web/ui/'+name],cwd=REPO)
            assert hashlib.sha256(committed).hexdigest()==asset['committed_sha256']
            assert source.read_bytes().replace(b'\r\n',b'\n')==committed
        for shot in r['screenshots']:assert (case/shot).is_file()
        cases[label]=(case,r,audit_case(case,fixture if label=='authored' else None))
    authored=cases['authored'][1];ordinary=cases['ordinary'][1]
    assert authored['fixture']['source']==fixture_manifest
    expected_names=['uk-1990-same-day-death','uk-2027-term-limit']
    assert [c['name'] for c in fixture_manifest['cases']]==[c['name'] for c in authored['cases']]==expected_names
    actual_fixture_hashes={'manifest':sha(fixture/'manifest.json')}
    for native,browser in zip(fixture_manifest['cases'],authored['cases']):
        name=native['name'];assert native['file']==name+'.json' and native['player']=='UK'
        assert native['days_advanced']==browser['days_advanced']==0 and browser['passed']
        actual_fixture_hashes[name]=sha(fixture/native['file'])
        assert {r['width'] for r in browser['campaign_layout']}=={1440,390,320}
        assert {r['width'] for r in browser['reference_layout']}=={1440,390,320}
        assert browser['continued_layout']['width']==320
        assert native['government']['party_leadership']==native['campaign']
        assert native['reference']['eligibility_context']=='historical_reference'
        assert native['reference']['enabled'] is False
        assert native['reference']['executive_person'] is None
        assert native['reference']['executive_mortality_supported'] is False
        source_party=next(p for p in native['reference']['parties'] if p['party_id']==native['party_id'])
        for p in native['reference']['parties']:
            assert p['campaign']==p['future_candidates']==p['future_preview']==[]
            for key in ['historical','uncertain_historical','eligible']:
                assert all('campaign_succession' not in person for person in p[key])
        if name=='uk-1990-same-day-death':
            assert any(p['person']['id']==native['focus_person'] and p['person']['name']=='Margaret Thatcher' for p in source_party['historical'])
            assert any(p['person']['id']==native['focus_person'] for p in source_party['eligible'])
        else:
            current=next(p for p in native['campaign']['parties'] if p['party_id']==native['party_id'])
            holder=next(p for p in current['campaign'] if p['person']['id']==native['focus_person'])
            status=holder['campaign_succession']
            assert status['party']['status']=='recorded_holder' and status['national_office']['status']=='excluded'
            assert status['office_excluded_date']=='2027-01-02' and status['death_date'] is None
            assert holder['executive_eligibility']['authorized'] is True and holder['person']['fiction']['origin']=='fictional_successor'
            for layout in browser['campaign_layout']+[browser['continued_layout']]:
                for lane in ['holders','future']:
                    shown=next(p for p in layout[lane] if p['person']==native['focus_person'])
                    assert shown['status']==status
                    assert all(not leaf['errors'] for leaf in shown['leaves'])
        # Archive worker separately proves every typed native world byte. Also
        # preserve and verify the outer campaign's complete recorded history.
        original=read(fixture/native['file'])
        assert original['format']=='spheres-campaign' and original['saved_date']==native['date']
        for stage in ['loaded','browsed','load-cancelled','reloaded','continued']:
            capture=cases['authored'][0]/'server/captures'/('s10f-audit-'+name+'-'+stage+'.json')
            value=read(capture)
            for key in ['format','version','history','history_epoch','log','saved_date','player']:
                assert value[key]==original[key],(capture,key)
        assert browser['final_capture']=='captures/s10f-audit-'+name+'-continued.json'
    assert authored['fixture']['sha256']==actual_fixture_hashes
    assert authored['commands']==authored['previews']==authored['advances']==[] and authored['days_advanced']==0
    assert cases['authored'][2]=={'inspections':12,'exact_comparisons':10,'ignored_paths':[]}
    # This existing ordinary driver records all native checkpoint dates. It
    # has no advance-telemetry field; do not invent one in the published proof.
    dates={ordinary['cancelled_policy_review']['date_label'],ordinary['confirmed_policy_review']['date_label'],ordinary['held_government_review']['date_label'],ordinary['confirmed_government_review']['date_label'],ordinary['government_result']['date'],ordinary['final_state']['date']}
    assert len(dates)==1 and ordinary['final_state']['date']=='1 Jan 1990'
    launch=read(BASE/'S10f-review-launch.json')
    assert launch['runtime_revision']==PIN and launch['executable_sha256']==binary['binary']['sha256']
    assert launch['native_verification']['exact_bytes_match_final_browser'] and launch['native_verification']['ignored_paths']==[]
    assert launch['date']==ordinary['final_state']['date'] and launch['policy']==ordinary['final_state']['policy']
    for part in launch['evidence']:assert sha(part['path'])==part['sha256']
    assert sha(launch['save_copy']['source'])==sha(launch['save_copy']['copy'])==launch['save_copy']['sha256']
    preserve_before=passed(BASE/'evidence/S10f-preservation-before.json');preserve=passed(BASE/'evidence/S10f-preservation-final.json')
    assert preserve_before['baseline_sha256']==preserve['baseline_sha256']
    visual=passed(BASE/'evidence/S10f-visual-review.json');assert visual['runtime_revision']==PIN
    assert isinstance(visual['screenshots'],list) and visual['screenshots'],'Actual visual review images are required'
    for image in visual['screenshots']:assert sha(image['path'])==image['sha256']
    items=[]
    def add(p,name):
        p=pathlib.Path(p);assert p.is_file() and not p.is_symlink()
        assert p.suffix.lower() not in {'.exe','.pdb','.dll'}
        items.append({'source':str(p),'name':name})
    def tree(p,name):
        assert p.is_dir()
        for file in sorted(p.rglob('*')):
            assert not file.is_symlink()
            if file.is_file():add(file,name+'/'+file.relative_to(p).as_posix())
    for stem in ['S10-f-web','S10-f-node','S10-f-leadership','S10-f-agency','S10-f-binary','S10-f-browser']:
        for ext in ['.json','.log']:add(BASE/'evidence'/(stem+ext),'windows/'+stem+ext)
    for source,label in [('S10f-linux-final','linux'),('S10f-content-final','content'),('S10f-succession-fixture-2','authored-native-fixture'),('S10f-succession-qualified','authored-browser'),('S10-browser-f-browser','ordinary-browser')]:
        tree(BASE/'evidence'/source,label)
    for name in ['S10f-preservation-before.json','S10f-preservation-final.json','S10f-visual-review.json']:add(BASE/'evidence'/name,'review/'+name)
    add(BASE/'S10f-review-launch.json','review/launch.json')
    tree(pathlib.Path(launch['directory'])/'native-verification','review/native-verification')
    for name in ['run-s10f-check.py','run-s10f-linux.py','run-s10f-content.py','run-s10f-succession.py','launch-s10f-review.py','verify-s08-preservation.py','collect-s09-evidence.py','publish-s10f.py','verify-s10f-publication.py']:
        add(BASE/name,'runners/'+name)
    preflight=BASE/'evidence/S10f-preflight-native.log'
    if preflight.exists():add(preflight,'development/S10f-preflight-native.log')
    # The first run detected a font-metric false positive. Preserve its report
    # and screenshot as development evidence without duplicating its archives.
    failed=BASE/'evidence/S10f-succession-final'
    failed_wrapper=read(failed/'result.json')
    assert failed_wrapper['passed'] is False and failed_wrapper['revision']==failed_wrapper['driver_revision']==PIN
    assert failed_wrapper['runner_sha256']==sha(BASE/'run-s10f-succession.py')
    assert [(c['name'],c['exit_code']) for c in failed_wrapper['checks']]==[('export',0),('browser',1)]
    for check in failed_wrapper['checks']:assert sha(failed/check['log'])==check['log_sha256']
    for name in ['result.json','browser.log','export.log']:
        if (failed/name).exists():add(failed/name,'development/authored-initial/'+name)
    failed_results=list((failed/'browser').glob('*/result.json'));assert len(failed_results)==1
    failed_browser=read(failed_results[0])
    assert failed_browser['passed'] is False and failed_browser['build']['revision']==PIN
    assert failed_browser['test_source']['revision']==PIN
    assert 'Margaret Thatcher' in failed_browser['failure'] and 'clipped text' in failed_browser['failure']
    for name in ['result.json','failure.png']:
        add(failed_results[0].parent/name,'development/authored-initial/browser/'+name)
    selection=BASE/'evidence/S10f-selection.json';assert not selection.exists()
    write_text(selection,json.dumps(items,indent=2)+'\n')
    collector=module('collector','collect-s09-evidence.py')
    collector.preflight(items,DEST/'evidence')
    DEST.mkdir()
    write_text(DEST/'.gitattributes','evidence/** -text -whitespace\n')
    collector.collect(selection,DEST/'evidence')
    inventory=read(DEST/'evidence/inventory.json')
    inventory['scope']='Complete selected S10.f native/interface/content logs, two authored succession cases and ordinary France browser archives with exact whole-world comparisons. Both browser journeys advance zero days. Runtime and native/Node/content/ordinary checks use the recorded runtime pin; the qualified authored browser uses a separately recorded driver revision whose only difference is its text-visibility assertion. Initial failed browser reports are development evidence, not qualification. Compiled binaries are identified by SHA and not bundled.'
    write_text(DEST/'evidence/inventory.json',json.dumps(inventory,indent=2)+'\n')
    manifest={'format':'spheres-s10f-increment/v1','status':'increment_complete_parent_open',
        'created_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'runtime_revision':PIN,'logic_revision':PIN,'driver_revision':driver_revision,'final_delta':final_delta,
        'binary_sha256':binary['binary']['sha256'],'c01_complete':False,'s10_complete':False,'g2_earned':False,'s11_started':False,
        'historical_role_or_gameplay_authorization_changed':False,'browser_elapsed_days':0,'authored_browser_elapsed_days':0,'ordinary_browser_elapsed_days':0,
        'qualification':{'windows_web':web['totals'],'windows_node':node['totals'],'windows_leadership':leadership['totals'],'windows_agency':agency['totals'],
            'linux_checks':[{'name':c['name'],'totals':c['totals']} for c in linux['checks']],
            'content_tests':sum(test_totals.values()),'content_test_groups':test_totals,'content_commands':len(content['checks']),
            'authored_browser':cases['authored'][2],'ordinary_browser':cases['ordinary'][2],
            'authored_cases':len(fixture_manifest['cases']),'authored_case_names':[c['name'] for c in fixture_manifest['cases']],
            'authored_commands':len(authored['commands']),'authored_previews':len(authored['previews']),
            'authored_native_fixture_sha256':actual_fixture_hashes,
            'authored_scope':fixture_manifest['scope'],'ordinary_scope':ordinary['fixture'],'new_long_campaign_or_performance_claim':False,
            'not_refreshed':['Full Linux Node','Eight-country startup matrix','External old-save fixture matrix','Long campaign and Russia activation']},
        'research_counts':read(REPO/'docs/campaign-certification/C01/research-index.json')['counts'],
        'review_url':launch['url'],'evidence':{'inventory_sha256':sha(DEST/'evidence/inventory.json'),'logical_files':len(inventory['files']),'stored_bytes':inventory['stored_bytes']}}
    write_text(DEST/'manifest.json',json.dumps(manifest,indent=2)+'\n')
    print(json.dumps(manifest,indent=2),flush=True)
if __name__=='__main__':main()
