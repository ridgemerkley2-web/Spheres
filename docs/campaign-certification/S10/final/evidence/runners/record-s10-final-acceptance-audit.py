"""Outside-only S10 acceptance/source receipt; never awards a gate or reruns tests."""
import datetime, gzip, hashlib, json, pathlib, re, subprocess
BASE = pathlib.Path(__file__).resolve().parent
REPO = BASE / 'integration'
PIN = 'bcdf72bcbe8947966359deb95f715cb526a68def'
OUT = BASE / 'evidence/S10-final-acceptance-audit.json'
def digest(data): return hashlib.sha256(data).hexdigest()
def git(*args): return subprocess.check_output(['git','-c','core.longpaths=true',*args],cwd=REPO)
def read(path): return json.loads(path.read_text(encoding='utf-8-sig'))
def file_record(path):
    data=path.read_bytes()
    return {'path':str(path.resolve()),'bytes':len(data),'sha256':digest(data)}
assert git('rev-parse','HEAD').decode().strip()==PIN
assert not git('status','--porcelain'), 'Receipt requires the exact clean qualification source'

def equality(prior, paths, label):
    args=['diff','--no-ext-diff','--no-textconv','--binary',prior,PIN,'--',*paths]
    patch=git(*args)
    names=git('diff','--name-only',prior,PIN,'--',*paths).decode().splitlines()
    before=git('ls-tree','-r',prior,'--',*paths)
    after=git('ls-tree','-r',PIN,'--',*paths)
    return {'label':label,'prior_revision':prior,'current_revision':PIN,
        'pathspecs':paths,'git_arguments':args,'git_diff_bytes':len(patch),
        'git_diff_sha256':digest(patch),'matches':not patch,'changed_paths':names,
        'prior_ls_tree_sha256':digest(before),'current_ls_tree_sha256':digest(after),
        'prior_tree_entries':len(before.splitlines()),'current_tree_entries':len(after.splitlines()),
        'comparison':'Committed Git blob bytes, without checkout newline conversion or textconv. Empty binary diff and identical tree listing are required for matches.'}

runtime=['Cargo.toml','Cargo.lock','spheres-sim','spheres-web']
prior_specs={
 'd': {'driver':'tools/ui/ci-diplomatic-inbox.cjs','feature':['spheres-sim/src/agency.rs','spheres-web/src/s10d_inbox_fixture_tests.rs'],
       'claim':'Retained authored inbox integration evidence. Only the listed native agency/fixture and driver scopes are equal; later shared decision code, statecraft and agency presentation changed. Prior screenshots are historical, not current-source visual qualification.'},
 'e': {'driver':'tools/ui/ci-diplomatic-commitments.cjs','feature':['spheres-sim/src/agency.rs','spheres-sim/src/statecraft.rs','spheres-web/src/diplomatic_commitments.rs','spheres-web/src/s10e_commitment_fixture_tests.rs'],
       'claim':'Retained authored commitments and one-day integration evidence. Listed commitment/agency/statecraft scopes are equal; later leadership and shared decision presentation changed. Prior browser evidence is not a new rerun or whole-runtime equality claim.'},
 'f': {'driver':'tools/ui/ci-succession-context.cjs','feature':['spheres-sim','spheres-web/src/government_view.rs','spheres-web/src/person_portraits.rs','spheres-web/src/s10_government_tests.rs','spheres-web/src/s10f_succession_fixture_tests.rs','spheres-web/ui/index.html','spheres-web/ui/government-ui.js','spheres-web/ui/government-ui.css'],
       'claim':'Same-source reuse of the listed succession model, government presentation and fixture paths. The later sanctions module changed shared server/agency paths, so this is scoped reuse, not an assertion that the complete F runtime equals current source.'},
 'g': {'driver':'tools/ui/ci-sanctions-desk.cjs','feature':runtime,
       'claim':'Same-source reuse of the complete game runtime and listed browser driver dependencies for authored sanctions confirmation/save continuity. The old execution remains a retained passing run, not a newly executed browser or performance test.'},
}
def driver_dependencies(entry, rev):
    found=set()
    def visit(name):
        if name in found: return
        found.add(name)
        body=git('show',rev+':'+name).decode('utf8')
        for ref in re.findall(r"require\(['\"](\.[^'\"]+)['\"]\)",body):
            joined=(pathlib.PurePosixPath(name).parent/ref).as_posix()
            parts=[]
            for part in joined.split('/'):
                if part=='..': parts.pop()
                elif part!='.': parts.append(part)
            target='/'.join(parts)
            if not pathlib.PurePosixPath(target).suffix: target+='.js'
            visit(target)
    visit(entry)
    return sorted(found)

retained=[]
for key,spec in prior_specs.items():
    directory=REPO/'docs/campaign-certification/S10'/key
    manifest_path=directory/'manifest.json'; inventory_path=directory/'evidence/inventory.json'
    manifest=read(manifest_path); inventory=read(inventory_path)
    rev=manifest['runtime_revision']; driver_rev=manifest.get('driver_revision',rev)
    # D and E published their successful authored browser source in its own result.
    result_rows=[r for r in inventory['files'] if r['logical_path'].startswith('authored-browser/') and r['logical_path'].endswith('result.json')]
    results=[]
    for row in result_rows:
        path=directory/'evidence'/row['logical_path']
        assert row['encoding'] in ('raw','gzip') and row['bytes']<20000000
        stored=[]; chunks=[]
        for part in row['storage']:
            storage_path=directory/'evidence'/part['file']; actual=file_record(storage_path)
            assert actual['sha256']==part['sha256'] and actual['bytes']==part['bytes']
            stored.append(actual); chunks.append(storage_path.read_bytes())
        body=b''.join(chunks)
        if row['encoding']=='gzip': body=gzip.decompress(body)
        assert digest(body)==row['sha256'] and len(body)==row['bytes']
        data=json.loads(body.decode('utf-8-sig'))
        assert data.get('passed') is True, str(path)
        results.append({'logical_path':row['logical_path'],'sha256':digest(body),'bytes':len(body),
            'encoding':row['encoding'],'storage':stored,'reconstruction_verified':True,'passed':True})
        candidate=data.get('test_source',{}).get('revision')
        if candidate: driver_rev=candidate
    comparisons=[equality(rev,runtime,'complete_game_runtime'),equality(rev,spec['feature'],'scoped_feature_source')]
    dependencies=driver_dependencies(spec['driver'],driver_rev)
    comparisons.append(equality(driver_rev,dependencies,'authored_driver_direct_relative_require_graph'))
    retained.append({'increment':'S10.'+key,'runtime_revision':rev,'driver_revision':driver_rev,
        'new_execution':False,'scope':spec['claim'],'report':file_record(directory/'README.md'),
        'manifest':file_record(manifest_path),'inventory':file_record(inventory_path),
        'authored_results':results,'comparisons':comparisons,
        'original_qualification':{k:v for k,v in manifest['qualification'].items() if k.startswith('authored') or k.startswith('performance')},
        'elapsed_days':manifest.get('authored_browser_elapsed_days',manifest.get('browser_elapsed_days'))})

lanes={name:[] for name in ['web','leadership','agency','binary','matrix','browser']}
for path in sorted((BASE/'evidence').glob('S10-*.json')):
    if path==OUT: continue
    try: data=read(path)
    except (ValueError,OSError): continue
    if data.get('revision')!=PIN: continue
    command=data.get('command',[])
    lane=None
    if command[:1]==['node']:
        lane={'tools/ui/run-unit.cjs':'node','tools/ui/ci-government-country-matrix.cjs':'matrix','tools/ui/ci-government-decisions.cjs':'browser'}.get(command[1])
    elif command[:2]==['cargo','build']: lane='binary'
    elif command[:2]==['cargo','test']:
        if any('s10g_mature_open_route_preview_latency' in p for p in command): lane='performance'
        elif 'party_leadership::' in command: lane='leadership'
        elif 'agency' in command: lane='agency'
        elif 'spheres-web' in command: lane='web'
    if lane in lanes:
        log=path.with_suffix('.log'); assert log.is_file()
        assert file_record(log)['sha256']==data['log_sha256']
        browser_results=[]
        if lane in ('matrix','browser'):
            for line in log.read_text(encoding='utf-8-sig').splitlines():
                try: summary=json.loads(line)
                except ValueError: continue
                if not isinstance(summary,dict) or not summary.get('result'): continue
                result_path=pathlib.Path(summary['result']).resolve()
                assert result_path.is_relative_to(BASE/'evidence')
                result=read(result_path); identity=result.get('runtime_revision',result.get('test_source',{}).get('revision'))
                assert identity==PIN
                assert result.get('passed') is True
                hashed=file_record(result_path)
                if summary.get('result_sha256'): assert hashed['sha256']==summary['result_sha256']
                browser_results.append({**hashed,'passed':True,'nation':result.get('nation'),
                    'scope':result.get('scope'),'full_eight_country_matrix':result.get('full_eight_country_matrix'),
                    'final_state':result.get('final_state'),'checks':result.get('checks')})
            assert browser_results
            if lane=='matrix':
                assert len(browser_results)==9 and browser_results[-1]['full_eight_country_matrix'] is True
                assert {r['nation'] for r in browser_results[:-1]}=={'France','Japan','India','Brazil','SouthAfrica','Tonga','SaudiArabia','USSR'}
        lanes[lane].append({**file_record(path),'log':file_record(log),'passed':data.get('passed') is True,
            'totals':data.get('totals'),'finished_utc':data.get('finished_utc'),
            'same_clean_source':data.get('revision_after')==PIN and data.get('clean_after') is True,
            'browser_results':browser_results})

checks=[]
for name,rows in lanes.items():
    checks.append({'id':'windows_'+name,'status':'passed' if rows and all(r['passed'] and r['same_clean_source'] for r in rows) else 'pending',
                   'receipts':rows,'scope':'Actual current-pin run receipts only. Similar historical filenames at other revisions are excluded.'})
for name,path in [('linux',BASE/'evidence/S10-final-linux/result.json'),('content_ui',BASE/'evidence/S10-final-content-ui/result.json')]:
    item={'id':name,'status':'pending','expected_path':str(path)}
    if path.is_file():
        data=read(path); actual_pin=data.get('revision',data.get('source_revision'))
        assert actual_pin==PIN
        item.update(receipt=file_record(path),record_status=data.get('status'),checks=data.get('checks',[]))
        logs=[]
        for check in data.get('checks',[]):
            log=path.parent/check['log']; hashed=file_record(log)
            assert hashed['sha256']==check['log_sha256']
            logs.append(hashed)
        item['logs']=logs
        if data.get('passed') is True: item['status']='passed'
    checks.append(item)
check_map={row['id']:row['status'] for row in checks}
def clause(number,text,required,retained_ids,oracles):
    return {'number':number,'text':text,'status':'passed' if all(check_map[x]=='passed' for x in required) else 'pending',
            'required_current_checks':required,'retained_evidence':retained_ids,'decisive_oracles':oracles}
clauses=[
 clause(1,'Test parliamentary, presidential, authoritarian and monarchical/institutional cases through review, confirmation and dated result.',
        ['windows_web','windows_matrix','linux'],[],[
        'tools/ui/ci-government-country-matrix.cjs: eight ordinary Jan-1990 starts; Japan/India parliamentary, France/Brazil presidential, SouthAfrica/USSR authoritarian, Tonga/SaudiArabia institutional; exact quoted costs/effects and dated results; zero elapsed days and no grants.',
        'spheres-web/src/s10_government_tests.rs: four-role legal election/institution fixtures, exact native results, role permissions and reload.']),
 clause(2,'Preserve saved incumbents; historical browsing and future candidate eligibility do not replace an officeholder automatically.',
        ['windows_web','windows_leadership','windows_matrix','linux'],['S10.f'],[
        's10_historical_browsing_and_future_eligibility_never_replace_saved_incumbents; s10_actual_future_succession_respects_executive_review_and_recorded_monarchy_heir.',
        'party_leadership library: same-date independent references, native death/exclusion dates, future cutoff, party/executive distinction and saved officeholders.',
        'S10.f two authored browser worlds; viewing, reference switching and save continuity preserve exact world, log and history. No elapsed-campaign claim.']),
 clause(3,'Validate offers, deadlines, standing policies, sanctions, party/executive roles, unaffordable/stale reviews and foreign inspection.',
        ['windows_web','windows_agency','windows_browser','windows_matrix','linux'],['S10.d','S10.e','S10.f','S10.g'],[
        's10_decision_tests, decision_review S10.d tests, diplomatic_commitments and diplomatic_sanctions tests: protected token/receipt paths, exclusive deadlines, standing policies, native caps and persistent restrictions.',
        'Ordinary France browser: actual second-tab change, stale refusal before mutation, fresh confirmation and exact Save/Load/Continue.',
        'Eight-country matrix: foreign controls disabled and viewing/cancellation pure; retained authored D/E/G browser results keep their original source/elapsed-time scope.'])]

record={'format':'spheres-s10-final-acceptance-source-audit/v1','created_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),
 'source_revision':PIN,'source_clean':True,'runner':file_record(pathlib.Path(__file__)),
 'status':'passed' if all(row['status']=='passed' for row in checks) else 'pending_qualification',
 'passed':all(row['status']=='passed' for row in checks),
 'scope':'Independent acceptance and committed-source audit. New final-run receipts are distinguished from retained prior runs. No tests are run by this script; prior proof bytes are only read and hashed.',
 'gate_decision':'not_awarded_by_this_receipt','s10_complete':False,'g2_earned':False,'c01_complete':False,'s11_started':False,
 'new_performance_claim':False,'new_long_campaign_claim':False,'new_historical_coverage_claim':False,
 'c01_dependency':'At this checked source the approved pathway lists C01. Exhaustive censuses and leadership histories remain open. Gameplay acceptance evidence cannot silently waive this dependency; root must separately record any explicit scope decision.',
 'acceptance_clauses':clauses,'current_qualification':checks,'retained_authored_evidence':retained,
 'narrative':'All three written gameplay clauses have concrete native and browser oracles. The final eight-country and France two-tab runs refresh gameplay acceptance on the final clean pin. D/E source differences prevent wholesale current-UI reuse; their results retain their historical source scope. F permits narrower exact succession-source reuse. G permits exact whole-game-runtime reuse. This receipt adds no campaign days, historical coverage, art, Russia activation, long-run certification or release gate.'}
OUT.write_text(json.dumps(record,indent=2)+'\n',encoding='utf8',newline='\n')
print(json.dumps({'path':str(OUT),'status':record['status'],'sha256':file_record(OUT)['sha256'],
                  'checks':check_map,'source_matches':{r['increment']:{x['label']:x['matches'] for x in r['comparisons']} for r in retained}},indent=2))
