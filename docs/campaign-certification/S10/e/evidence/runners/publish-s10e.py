"""Publish a NEW bounded S10.e checkpoint only after its declared proofs pass."""
import datetime,hashlib,importlib.util,json,pathlib,re,subprocess
BASE=pathlib.Path(__file__).resolve().parent
REPO=BASE/'integration'
DEST=REPO/'docs/campaign-certification/S10/e'
PIN='99b0b9d4a46a98a2c0cf7886583fe13e568dda64'

def read(p):return json.loads(pathlib.Path(p).read_text(encoding='utf-8-sig'))
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
    assert r['runner_sha256']==sha(BASE/'run-s10-check.py')
    log=p.with_suffix('.log').read_text(encoding='utf8',errors='replace')
    if lane in ('web','agency'):assert native_totals(log)==r['totals']
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
    assert git('rev-parse','HEAD')==PIN and not git('status','--porcelain')
    assert not DEST.exists(),'Publish before writing S10/e docs; never replace prior evidence'
    proofs={lane:proof('S10-e-'+lane+'.json',lane) for lane in ['web','node','agency','binary','browser']}
    web,node,agency,binary=[proofs[k] for k in ['web','node','agency','binary']]
    assert proofs['browser']['binary']['sha256']==proofs['browser']['binary_sha256_before']==binary['binary']['sha256']
    for key in ['test_binary']:
        assert sha(web[key]['path'])==web[key]['sha256']
    assert sha(binary['binary']['path'])==binary['binary']['sha256']
    linux_dir=BASE/'evidence/S10e-linux-final';linux=passed(linux_dir/'result.json')
    checks=checked_logs(linux_dir,linux,'run-s10e-linux.py');assert len(checks)==2
    assert linux['source_before']=={'head':PIN,'status':''}
    for c in checks:
        assert c['source_after']=={'head':PIN,'status':''}
        assert native_totals((linux_dir/c['log']).read_text(encoding='utf8',errors='replace'))==c['totals']
    assert {c['name'] for c in checks}=={'web','agency'}
    content_dir=BASE/'evidence/S10e-content-final';content=passed(content_dir/'result.json')
    checks=checked_logs(content_dir,content,'run-s10e-content.py')
    assert content['clean_after'] and content['revision_after']==PIN and len(checks)==10
    expected_tests={'india-tests','japan-tests','census-tests','france-tests','portrait-tests'}
    test_totals={}
    for c in checks:
        if c['name'] in expected_tests:
            log=(content_dir/c['log']).read_text(encoding='utf8',errors='replace')
            counts=re.findall(r'Ran (\d+) tests? in ',log);assert len(counts)==1 and int(counts[0])>0
            assert re.search(r'^OK\s*$',log,re.M) and not re.search(r'^FAILED\b',log,re.M)
            test_totals[c['name']]=int(counts[0])
    assert set(test_totals)==expected_tests
    authored_dir=BASE/'evidence/S10e-commitments-final'
    wrapper=passed(authored_dir/'result.json')
    checked_logs(authored_dir,wrapper,'run-s10e-commitments.py')
    assert wrapper['clean_after'] and wrapper['driver_revision']==wrapper['driver_revision_after']==PIN
    assert wrapper['binary_sha256_after']==binary['binary']['sha256']
    assert wrapper['checks'][-1]['name']=='browser'
    fixture=BASE/'evidence/S10e-commitments-fixture-2';fixture_manifest=read(fixture/'manifest.json')
    assert fixture_manifest['fixture']=='s10e-authored-native-diplomatic-commitments'
    assert fixture_manifest['compiled_revision']==PIN[:12] and fixture_manifest['days_advanced']==1 and fixture_manifest['reply_days_advanced']==0
    cases={}
    for label,folder,driver in [('authored',authored_dir/'browser','ci-diplomatic-commitments.cjs'),('ordinary',BASE/'evidence/S10-browser-e-browser','ci-government-decisions.cjs')]:
        results=list(folder.glob('*/result.json'));assert len(results)==1
        case=results[0].parent;r=passed(results[0])
        assert r['build']['revision']==PIN and r['build']['binary_sha256']==binary['binary']['sha256']
        assert r['test_source']['revision']==PIN and r['test_source']['runtime_source_equal'] and not r['errors']
        assert r['test_source']['driver_sha256']==sha(REPO/'tools/ui'/driver)
        assert r['test_source']['review_assertions_sha256']==sha(REPO/'tools/ui/government-review-assertions.cjs')
        for shot in r['screenshots']:assert (case/shot).is_file()
        cases[label]=(case,r,audit_case(case,fixture if label=='authored' else None))
    authored=cases['authored'][1];ordinary=cases['ordinary'][1]
    assert authored['fixture']['source']==fixture_manifest
    actual_fixture_hashes={'manifest':sha(fixture/'manifest.json'),'before':sha(fixture/'before.json'),'expected_after':sha(fixture/'expected-after.json'),'expected_day':sha(fixture/'expected-after-day.json')}
    assert authored['fixture']['sha256']==actual_fixture_hashes
    assert len(authored['cancelled_reviews'])==2 and {q['command']['accept'] for q in authored['cancelled_reviews']}=={True,False}
    assert len(authored['commands'])==1 and authored['commands'][0]['commands']==[fixture_manifest['command']]
    assert len(authored['advances'])==1 and authored['advances'][0]['days']==1 and authored['advances'][0]['commands']==[]
    assert authored['days_advanced']==1 and authored['reply_days_advanced']==0
    assert authored['settled_notice']['focused'] and authored['settled_notice']['visible']
    assert {r['width'] for r in authored['initial_layout']}=={1440,390,320}
    assert {r['width'] for r in authored['joined_layout']}=={1440,390,320}
    assert {r['width'] for r in authored['context_layout']}=={1440,390,320}
    assert len(authored['request_links'])==3 and all(r['focused'] and r['previews_sent']==r['commands_sent']==0 for r in authored['request_links'])
    # This existing ordinary driver records all native checkpoint dates. It
    # has no advance-telemetry field; do not invent one in the published proof.
    dates={ordinary['cancelled_policy_review']['date_label'],ordinary['confirmed_policy_review']['date_label'],ordinary['held_government_review']['date_label'],ordinary['confirmed_government_review']['date_label'],ordinary['government_result']['date'],ordinary['final_state']['date']}
    assert len(dates)==1 and ordinary['final_state']['date']=='1 Jan 1990'
    launch=read(BASE/'S10e-review-launch.json')
    assert launch['runtime_revision']==PIN and launch['executable_sha256']==binary['binary']['sha256']
    assert launch['native_verification']['exact_bytes_match_final_browser'] and launch['native_verification']['ignored_paths']==[]
    assert launch['date']==ordinary['final_state']['date'] and launch['policy']==ordinary['final_state']['policy']
    for part in launch['evidence']:assert sha(part['path'])==part['sha256']
    assert sha(launch['save_copy']['source'])==sha(launch['save_copy']['copy'])==launch['save_copy']['sha256']
    preserve_before=passed(BASE/'evidence/S10e-preservation-before.json');preserve=passed(BASE/'evidence/S10e-preservation-final.json')
    assert preserve_before['baseline_sha256']==preserve['baseline_sha256']
    visual=passed(BASE/'evidence/S10e-visual-review.json');assert visual['runtime_revision']==PIN
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
    for stem in ['S10-e-web','S10-e-node','S10-e-agency','S10-e-binary','S10-e-browser']:
        for ext in ['.json','.log']:add(BASE/'evidence'/(stem+ext),'windows/'+stem+ext)
    for source,label in [('S10e-linux-final','linux'),('S10e-content-final','content'),('S10e-commitments-fixture-2','authored-native-fixture'),('S10e-commitments-final','authored-browser'),('S10-browser-e-browser','ordinary-browser')]:
        tree(BASE/'evidence'/source,label)
    for name in ['S10e-preservation-before.json','S10e-preservation-final.json','S10e-visual-review.json']:add(BASE/'evidence'/name,'review/'+name)
    add(BASE/'S10e-review-launch.json','review/launch.json')
    tree(pathlib.Path(launch['directory'])/'native-verification','review/native-verification')
    for name in ['run-s10-check.py','run-s10e-linux.py','run-s10e-content.py','run-s10e-commitments.py','launch-s10e-review.py','verify-s08-preservation.py','collect-s09-evidence.py','publish-s10e.py','verify-s10e-publication.py']:
        add(BASE/name,'runners/'+name)
    preflight=BASE/'evidence/S10e-preflight-native.log'
    if preflight.exists():add(preflight,'development/S10e-preflight-native.log')
    selection=BASE/'evidence/S10e-selection.json';assert not selection.exists()
    selection.write_text(json.dumps(items,indent=2)+'\n',encoding='utf8')
    collector=module('collector','collect-s09-evidence.py')
    collector.preflight(items,DEST/'evidence')
    DEST.mkdir()
    (DEST/'.gitattributes').write_text('evidence/** -text -whitespace\n',encoding='utf8')
    collector.collect(selection,DEST/'evidence')
    inventory=read(DEST/'evidence/inventory.json')
    inventory['scope']='Complete selected S10.e native/interface/content logs, final authored one-day and ordinary zero-day browser archives and exact whole-world comparisons. All qualification checks use the single pinned runtime; optional development logs are not qualification. Compiled binaries are identified by SHA and not bundled.'
    (DEST/'evidence/inventory.json').write_text(json.dumps(inventory,indent=2)+'\n',encoding='utf8')
    manifest={'format':'spheres-s10e-increment/v1','status':'increment_complete_parent_open',
        'created_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'runtime_revision':PIN,'logic_revision':PIN,'final_delta':[],
        'binary_sha256':binary['binary']['sha256'],'c01_complete':False,'s10_complete':False,'g2_earned':False,'s11_started':False,
        'historical_role_or_gameplay_authorization_changed':False,'browser_elapsed_days':1,'authored_browser_elapsed_days':1,'ordinary_browser_elapsed_days':0,
        'qualification':{'windows_web':web['totals'],'windows_node':node['totals'],'windows_agency':agency['totals'],
            'linux_checks':[{'name':c['name'],'totals':c['totals']} for c in linux['checks']],
            'content_tests':sum(test_totals.values()),'content_test_groups':test_totals,'content_commands':len(content['checks']),
            'authored_browser':cases['authored'][2],'ordinary_browser':cases['ordinary'][2],
            'authored_offers':len(fixture_manifest['before_agency']['offers']),'cancelled_choices':len(authored['cancelled_reviews']),
            'confirmed_replies':len(authored['commands']),'authored_native_fixture_sha256':actual_fixture_hashes,
            'authored_scope':fixture_manifest['scope'],'ordinary_scope':ordinary['fixture'],'new_long_campaign_or_performance_claim':False,
            'not_refreshed':['Full Linux Node','Eight-country startup matrix','External old-save fixture matrix','Long campaign and Russia activation']},
        'research_counts':read(REPO/'docs/campaign-certification/C01/research-index.json')['counts'],
        'review_url':launch['url'],'evidence':{'inventory_sha256':sha(DEST/'evidence/inventory.json'),'logical_files':len(inventory['files']),'stored_bytes':inventory['stored_bytes']}}
    (DEST/'manifest.json').write_text(json.dumps(manifest,indent=2)+'\n',encoding='utf8')
    print(json.dumps(manifest,indent=2),flush=True)
if __name__=='__main__':main()
