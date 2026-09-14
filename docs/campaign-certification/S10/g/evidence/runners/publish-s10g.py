"""Publish a NEW bounded S10.g checkpoint only after its declared proofs pass."""
import argparse,datetime,hashlib,importlib.util,json,pathlib,re,subprocess,sys,zipfile
BASE=pathlib.Path(__file__).resolve().parent
REPO=BASE/'integration'
DEST=REPO/'docs/campaign-certification/S10/g'
PIN=None

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
    assert r['runner_sha256']==sha(BASE/'run-s10g-check.py')
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

def campaign_envelope(path):
    value=read(path)
    assert set(value)=={'format','version','world','history','log','history_epoch','saved_date','player','saved_unix'}
    assert value['format']=='spheres-campaign' and value['version']==1
    assert type(value['saved_unix']) is int and value['saved_unix']>=0
    timestamp=value.pop('saved_unix');value.pop('world')
    encoded=json.dumps(value,sort_keys=True,separators=(',',':'),ensure_ascii=False,allow_nan=False).encode('utf8')
    return {'sha256':hashlib.sha256(encoded).hexdigest(),'bytes':len(encoded),'saved_unix':timestamp,'value':value}

def main():
    global PIN
    parser=argparse.ArgumentParser()
    parser.add_argument('runtime_pin');parser.add_argument('--driver-revision')
    parser.add_argument('--authored-dir',type=pathlib.Path,default=BASE/'evidence/S10g-sanctions-final')
    parser.add_argument('--export-proof',type=pathlib.Path)
    parser.add_argument('--performance-proof',type=pathlib.Path,default=BASE/'evidence/S10-g-performance.json')
    parser.add_argument('--development-dir',action='append',default=[],metavar='LABEL=PATH')
    args=parser.parse_args();PIN=args.runtime_pin;driver_revision=args.driver_revision or PIN
    assert re.fullmatch('[0-9a-f]{40}',PIN) and re.fullmatch('[0-9a-f]{40}',driver_revision)
    assert git('rev-parse','HEAD')==driver_revision and not git('status','--porcelain')
    final_delta=git('diff','--name-only',PIN,driver_revision).splitlines()
    assert final_delta in ([],['tools/ui/ci-sanctions-desk.cjs']),'Only an explicitly pinned browser-only assertion correction may differ'
    assert not git('diff','--name-only',PIN,driver_revision,'--','spheres-sim','spheres-cli','spheres-web','Cargo.toml','Cargo.lock')
    assert not DEST.exists(),'Publish into a new S10/g checkpoint; never replace evidence'
    proofs={lane:proof('S10-g-'+lane+'.json',lane) for lane in ['web','node','agency','binary','browser']}
    web,node,agency,binary=[proofs[k] for k in ['web','node','agency','binary']]
    assert sha(web['test_binary']['path'])==web['test_binary']['sha256']
    assert sha(binary['binary']['path'])==binary['binary']['sha256']
    assert proofs['browser']['binary']['sha256']==proofs['browser']['binary_sha256_before']==binary['binary']['sha256']
    linux_dir=BASE/'evidence/S10g-linux-final';linux=passed(linux_dir/'result.json')
    checks=checked_logs(linux_dir,linux,'run-s10g-linux.py');assert len(checks)==2
    assert linux['source_before']=={'head':PIN,'status':''}
    for check in checks:
        assert check['source_after']=={'head':PIN,'status':''}
        assert native_totals((linux_dir/check['log']).read_text(encoding='utf8',errors='replace'))==check['totals']
    assert {c['name'] for c in checks}=={'web','agency'}
    content_dir=BASE/'evidence/S10g-content-final';content=passed(content_dir/'result.json')
    checks=checked_logs(content_dir,content,'run-s10g-content.py')
    assert content['clean_after'] and content['revision_after']==PIN and len(checks)==12
    expected_tests={'tonga-tests','brazil-tests','india-tests','japan-tests','census-tests','france-tests','portrait-tests'}
    test_totals={}
    for check in checks:
        if check['name'] in expected_tests:
            log=(content_dir/check['log']).read_text(encoding='utf8',errors='replace');counts=re.findall(r'Ran (\d+) tests? in ',log)
            assert len(counts)==1 and int(counts[0])>0
            assert re.search(r'^OK\s*$',log,re.M) and not re.search(r'^FAILED\b',log,re.M)
            test_totals[check['name']]=int(counts[0])
    assert set(test_totals)==expected_tests and sum(test_totals.values())==66
    authored_dir=args.authored_dir.resolve();wrapper=passed(authored_dir/'result.json')
    checked_logs(authored_dir,wrapper,'run-s10g-sanctions.py')
    assert wrapper['clean_after'] and wrapper['driver_revision']==wrapper['driver_revision_after']==driver_revision
    assert wrapper['binary_sha256_after']==binary['binary']['sha256']
    names=[c['name'] for c in wrapper['checks']];assert names in (['export','browser'],['browser'])
    export_proof=args.export_proof.resolve() if args.export_proof else authored_dir/'result.json'
    export=read(export_proof);assert export['revision']==PIN and export['runner_sha256']==sha(BASE/'run-s10g-sanctions.py')
    exported=[c for c in export['checks'] if c['name']=='export'];assert len(exported)==1 and exported[0]['exit_code']==0
    export_log=export_proof.parent/exported[0]['log'];assert sha(export_log)==exported[0]['log_sha256']
    assert native_totals(export_log.read_text(encoding='utf8',errors='replace'))['passed']==1
    fixture=BASE/'evidence/S10g-sanctions-fixture-1';fixture_manifest=read(fixture/'manifest.json')
    assert fixture_manifest['fixture']=='s10g-authored-native-sanctions-desk' and fixture_manifest['version']==1
    assert fixture_manifest['compiled_revision']==PIN[:12] and fixture_manifest['days_advanced']==0 and fixture_manifest['player']=='France'
    assert [s['kind'] for s in fixture_manifest['steps']]==['lift','sanction','improve']
    cases={}
    for label,folder,driver in [('authored',authored_dir/'browser','ci-sanctions-desk.cjs'),('ordinary',BASE/'evidence/S10-browser-g-browser','ci-government-decisions.cjs')]:
        results=list(folder.glob('*/result.json'));assert len(results)==1
        case=results[0].parent;result=passed(results[0])
        assert result['build']['revision']==PIN and result['build']['binary_sha256']==binary['binary']['sha256']
        assert result['test_source']['revision']==(driver_revision if label=='authored' else PIN)
        assert result['test_source']['runtime_source_equal'] and not result['errors']
        assert result['test_source']['driver_sha256']==sha(REPO/'tools/ui'/driver)
        assert result['test_source']['review_assertions_sha256']==sha(REPO/'tools/ui/government-review-assertions.cjs')
        for name,asset in result['build']['assets'].items():
            source=REPO/'spheres-web/ui'/name
            assert source.is_file() and sha(source)==asset['served_sha256']==asset['checkout_sha256']
            committed=subprocess.check_output(['git','-c','core.longpaths=true','show',PIN+':spheres-web/ui/'+name],cwd=REPO)
            assert hashlib.sha256(committed).hexdigest()==asset['committed_sha256'] and source.read_bytes().replace(b'\r\n',b'\n')==committed
        for shot in result['screenshots']:assert (case/shot).is_file()
        cases[label]=(case,result,audit_case(case,fixture if label=='authored' else None))
    case,authored,authored_audit=cases['authored'];ordinary=cases['ordinary'][1]
    assert authored['fixture']['source']==fixture_manifest and pathlib.Path(authored['fixture']['manifest_path']).resolve()==(fixture/'manifest.json').resolve()
    files=['before.json','expected-after-lift.json','expected-after-sanction.json','expected-after-improve.json']
    assert fixture_manifest['before_file']==files[0] and [s['expected_file'] for s in fixture_manifest['steps']]==files[1:]
    actual_fixture_hashes={name:sha(fixture/name) for name in files};actual_fixture_hashes['manifest']=sha(fixture/'manifest.json')
    assert authored['fixture']['sha256']==actual_fixture_hashes
    assert authored['days_advanced']==0 and authored['advances']==authored['government_previews']==[]
    assert [c['commands'] for c in authored['commands']]==[[s['command']] for s in fixture_manifest['steps']]
    assert all(c['review_kind']=='decisions' for c in authored['commands'])
    assert [p['command'] for p in authored['previews']]==[fixture_manifest['steps'][0]['command']]+[s['command'] for s in fixture_manifest['steps']]
    assert len(authored['actions'])==3 and len(authored['envelope_comparisons'])==9
    assert authored_audit=={'inspections':13,'exact_comparisons':9,'ignored_paths':[]}
    assert {r['width'] for r in authored['initial_layout']}=={1440,390,320}
    assert {r['width'] for r in authored['cancelled_context']}=={1440,390,320}
    assert authored['continued_layout']['width']==320
    assert authored['foreign_inspection']['nation']=='Japan' and authored['foreign_inspection']['disabled_controls']>0
    assert authored['foreign_inspection']['previews_sent']==authored['foreign_inspection']['commands_sent']==0
    assert fixture_manifest['before']['political_capital']==100 and fixture_manifest['before']['relation']==20
    initial_sanctions={tuple(p) for p in fixture_manifest['before']['sanctions']}
    assert initial_sanctions=={('France','Japan'),('Japan','France'),('India','Japan'),('Brazil','France')}
    for index,(step,action,sent) in enumerate(zip(fixture_manifest['steps'],authored['actions'],authored['commands'])):
        assert action['kind']==step['kind'] and step['days_advanced']==0
        expected_quote={k:v for k,v in step['quote'].items() if k not in ('review_token','session_id')}
        assert {k:v for k,v in action['quote'].items() if k not in ('review_token','session_id')}==expected_quote
        assert sent['review_token']==action['quote']['review_token']
        assert step['after']['political_capital']==[97,91,89][index] and step['after']['relation']==[20,5,11][index]
        assert step['after']['trade_depth']==0.425
        assert {tuple(p) for p in step['after']['sanctions']}==initial_sanctions-({('France','Japan')} if index==0 else set())
        context=step['quote']['bilateral_context'];assert context['actor']=='France' and context['target']=='Japan'
        assert context['after']['their_sanctions'] is True and context['after']['your_sanctions'] is (index!=0)
        assert context['after']['trade_treaty']['saved'] is True and context['after']['trade_treaty']['depth']==0.425
        assert any(p['id']=='India' for p in context['after']['other_sanctioners'])
        assert context['after']['target_growth_drag']['annual_pp']>0
        assert action['notice']['focused'] and action['notice']['visible'] and action['notice']['text'].strip()
        assert action['layout']['width']==320
    # Independently re-read every full envelope and compare its exact fields.
    # Only timestamp metadata differs between two separate ordinary saves.
    mappings={'loaded':'before.json','cancelled':'before.json','lift':'expected-after-lift.json','sanction':'expected-after-sanction.json','improve':'expected-after-improve.json','foreign':'expected-after-improve.json','load-cancelled':'expected-after-improve.json','reloaded':'expected-after-improve.json','continued':'expected-after-improve.json'}
    for stage,oracle in mappings.items():
        captured=campaign_envelope(case/'server/captures'/('s10g-audit-'+stage+'.json'));expected_envelope=campaign_envelope(fixture/oracle)
        assert captured['value']==expected_envelope['value']
        assert captured['sha256']==expected_envelope['sha256'] and captured['bytes']==expected_envelope['bytes']
    for comparison in authored['envelope_comparisons']:
        left,right=[pathlib.Path(comparison[k]).resolve() for k in ['left','right']]
        assert all(p.is_relative_to(case.resolve()) or p.is_relative_to(fixture.resolve()) for p in [left,right])
        a,b=campaign_envelope(left),campaign_envelope(right)
        assert a['value']==b['value'] and a['sha256']==b['sha256']==comparison['sha256']
        assert a['bytes']==b['bytes']==comparison['bytes']
        assert a['saved_unix']==comparison['left_saved_unix'] and b['saved_unix']==comparison['right_saved_unix']
    assert authored['final_capture']=='captures/s10g-audit-continued.json'
    dates={ordinary['cancelled_policy_review']['date_label'],ordinary['confirmed_policy_review']['date_label'],ordinary['held_government_review']['date_label'],ordinary['confirmed_government_review']['date_label'],ordinary['government_result']['date'],ordinary['final_state']['date']}
    assert len(dates)==1 and ordinary['final_state']['date']=='1 Jan 1990'
    launch=read(BASE/'S10g-review-launch.json');assert launch['runtime_revision']==PIN and launch['executable_sha256']==binary['binary']['sha256']
    assert launch['native_verification']['exact_bytes_match_final_browser'] and launch['native_verification']['ignored_paths']==[]
    assert launch['date']==ordinary['final_state']['date'] and launch['policy']==ordinary['final_state']['policy']
    for part in launch['evidence']:assert sha(part['path'])==part['sha256']
    assert sha(launch['save_copy']['source'])==sha(launch['save_copy']['copy'])==launch['save_copy']['sha256']
    preserve_before=passed(BASE/'evidence/S10g-preservation-before.json');preserve=passed(BASE/'evidence/S10g-preservation-final.json');assert preserve_before['baseline_sha256']==preserve['baseline_sha256']
    visual=passed(BASE/'evidence/S10g-visual-review.json');assert visual['runtime_revision']==PIN and visual['screenshots']
    for image in visual['screenshots']:assert sha(image['path'])==image['sha256']
    performance=None
    if args.performance_proof:
        perf_path=args.performance_proof.resolve();performance=passed(perf_path)
        assert performance['revision']==performance['revision_after']==PIN and performance['clean_before'] and performance['clean_after']
        assert performance['exit_code']==0 and performance['runner_sha256']==sha(BASE/'run-s10g-check.py')
        assert sha(perf_path.with_suffix('.log'))==performance['log_sha256']
        assert native_totals(perf_path.with_suffix('.log').read_text(encoding='utf8',errors='replace'))==performance['totals']
        assert performance['source']['sha256_before']==performance['source']['sha256_after']==sha(performance['source']['path'])
        assert len(performance['measurements'])==1
        measured=performance['measurements'][0]
        log_measurements=[json.loads(line.split('S10G_SANCTIONS_PERF ',1)[1]) for line in perf_path.with_suffix('.log').read_text(encoding='utf8').splitlines() if 'S10G_SANCTIONS_PERF ' in line]
        assert log_measurements==performance['measurements'] and measured['passed'] is True
        assert measured['source_file_unchanged'] and measured['benchmark_world_unchanged'] and measured['log_history_unchanged']
        assert measured['days_advanced']==0 and measured['warmups']==3 and measured['samples']==21
        assert len(measured['samples_ms'])==21 and all(type(n) in (float,int) and n>0 for n in measured['samples_ms'])
        assert sorted(measured['samples_ms'])[19]==measured['p95_ms'] and max(measured['samples_ms'])==measured['max_ms']
        assert measured['p95_limit_ms']==300 and measured['max_limit_ms']==750
        assert measured['p95_ms']<=300 and measured['max_ms']<=750
        assert measured['nation']!=measured['target'] and measured['command']=={'kind':'improve','target':measured['target']}
        assert pathlib.Path(measured['source']).resolve()==pathlib.Path(performance['source']['path']).resolve()
        for state in ['before','after']:
            routes=measured['route_facts'][state];assert routes['enabled'] is True
            assert routes['outbound']['available'] is True and routes['inbound']['available'] is True
        for precondition in measured['authored_preconditions']:
            assert precondition['kind']=='political_capital_floor' and precondition['nation']==measured['nation']
            assert precondition['after']>precondition['before'] and precondition['reason']
        # Retain a hash-bound reference to the existing S08 campaign object;
        # reconstruct and compare its bytes instead of duplicating 134 MB.
        prior_inventory=REPO/'docs/campaign-certification/S08/evidence/inventory.json'
        prior=read(prior_inventory);source_sha=performance['source']['sha256_before']
        assert source_sha=='73a8ed5bd81502bf37d77f374c23f18ed96644a0032ba295317b25a0d01133f9'
        source_object=prior['compressed_objects'][source_sha]
        assert source_object['original_bytes']==performance['source']['bytes']==134238603
        digest=hashlib.sha256();offset=0
        for part in source_object['parts']:
            archive=prior_inventory.parent/part['file'];assert sha(archive)==part['stored_sha256'] and archive.stat().st_size==part['stored_bytes']
            assert part['source_offset']==offset
            with zipfile.ZipFile(archive) as package:raw=package.read(part['member'])
            assert len(raw)==part['source_bytes'] and hashlib.sha256(raw).hexdigest()==part['source_sha256']
            digest.update(raw);offset+=len(raw)
        assert digest.hexdigest()==source_sha and offset==source_object['original_bytes']
        performance_source={'checkpoint':'S08','inventory_path':prior_inventory.relative_to(REPO).as_posix(),'inventory_sha256':sha(prior_inventory),
            'source_path':performance['source']['path'],'source_sha256':source_sha,'source_bytes':offset,'compressed_object':source_object,'reconstruction_verified':True}
    assert performance is not None,'The bounded open-route preview timing is required for this increment'
    items=[]
    def add(p,name):
        p=pathlib.Path(p);assert p.is_file() and not p.is_symlink();assert p.suffix.lower() not in {'.exe','.pdb','.dll'}
        items.append({'source':str(p),'name':name})
    def tree(p,name):
        assert p.is_dir()
        for file in sorted(p.rglob('*')):
            assert not file.is_symlink()
            if file.is_file():add(file,name+'/'+file.relative_to(p).as_posix())
    for stem in ['S10-g-web','S10-g-node','S10-g-agency','S10-g-binary','S10-g-browser']:
        for ext in ['.json','.log']:add(BASE/'evidence'/(stem+ext),'windows/'+stem+ext)
    for source,label in [(linux_dir,'linux'),(content_dir,'content'),(fixture,'authored-native-fixture'),(authored_dir,'authored-browser'),(BASE/'evidence/S10-browser-g-browser','ordinary-browser')]:tree(source,label)
    if export_proof.parent!=authored_dir:
        add(export_proof,'native-export/result.json');add(export_log,'native-export/export.log')
    for name in ['S10g-preservation-before.json','S10g-preservation-final.json','S10g-visual-review.json']:add(BASE/'evidence'/name,'review/'+name)
    add(BASE/'S10g-review-launch.json','review/launch.json');tree(pathlib.Path(launch['directory'])/'native-verification','review/native-verification')
    for name in ['run-s10g-check.py','run-s10g-linux.py','run-s10g-content.py','run-s10g-sanctions.py','launch-s10g-review.py','verify-s08-preservation.py','collect-s09-evidence.py','publish-s10g.py','verify-s10g-publication.py']:add(BASE/name,'runners/'+name)
    if performance:
        add(args.performance_proof,'performance/result.json');add(args.performance_proof.with_suffix('.log'),'performance/native.log')
    for entry in args.development_dir:
        label,source=entry.split('=',1);assert re.fullmatch('[a-z0-9-]+',label);tree(pathlib.Path(source).resolve(),'development/'+label)
    selection=BASE/'evidence/S10g-selection.json';assert not selection.exists();write_text(selection,json.dumps(items,indent=2)+'\n')
    collector=module('collector','collect-s09-evidence.py');collector.preflight(items,DEST/'evidence');DEST.mkdir();write_text(DEST/'.gitattributes','evidence/** -text -whitespace\n');collector.collect(selection,DEST/'evidence')
    inventory=read(DEST/'evidence/inventory.json');inventory['scope']='Complete selected S10.g native/interface/content logs, four authored sanctions oracles and ordinary France browser archives. Exact typed whole-world comparisons omit no native fields; independent envelope checks retain complete recorded history and separately validate volatile save timestamps. Both browser journeys advance zero days. Any explicitly selected development attempts are not qualification. Compiled binaries are pinned by SHA and not bundled.';write_text(DEST/'evidence/inventory.json',json.dumps(inventory,indent=2)+'\n')
    manifest={'format':'spheres-s10g-increment/v1','status':'increment_complete_parent_open','created_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'runtime_revision':PIN,'logic_revision':PIN,'driver_revision':driver_revision,'final_delta':final_delta,
        'binary_sha256':binary['binary']['sha256'],'c01_complete':False,'s10_complete':False,'g2_earned':False,'s11_started':False,'historical_role_or_gameplay_authorization_changed':False,
        'browser_elapsed_days':0,'authored_browser_elapsed_days':0,'ordinary_browser_elapsed_days':0,
        'qualification':{'windows_web':web['totals'],'windows_node':node['totals'],'windows_agency':agency['totals'],'linux_checks':[{'name':c['name'],'totals':c['totals']} for c in linux['checks']],
            'content_tests':sum(test_totals.values()),'content_test_groups':test_totals,'content_commands':len(content['checks']),'authored_browser':authored_audit,'ordinary_browser':cases['ordinary'][2],
            'authored_cases':1,'authored_commands':len(authored['commands']),'authored_previews':len(authored['previews']),'authored_command_kinds':[s['kind'] for s in fixture_manifest['steps']],
            'authored_envelope_comparisons':len(authored['envelope_comparisons']),'authored_native_fixture_sha256':actual_fixture_hashes,'authored_scope':fixture_manifest['scope'],'ordinary_scope':ordinary['fixture'],
            'new_long_campaign_claim':False,'new_performance_claim':True,'performance_measurements':performance['measurements'],'performance_source_reference':performance_source,
            'not_refreshed':['Full Linux Node','Eight-country startup matrix','External old-save fixture matrix','Long campaign and Russia activation']},
        'research_counts':read(REPO/'docs/campaign-certification/C01/research-index.json')['counts'],'review_url':launch['url'],
        'evidence':{'inventory_sha256':sha(DEST/'evidence/inventory.json'),'logical_files':len(inventory['files']),'stored_bytes':inventory['stored_bytes']}}
    write_text(DEST/'manifest.json',json.dumps(manifest,indent=2)+'\n');print(json.dumps(manifest,indent=2),flush=True)
if __name__=='__main__':main()
