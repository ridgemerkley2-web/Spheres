"""Verify each S10.g stored byte, reconstruction, source and optional Git index."""
import argparse,gzip,hashlib,json,pathlib,re,subprocess
BASE=pathlib.Path(__file__).resolve().parent
REPO=BASE/'integration'
ROOT=REPO/'docs/campaign-certification/S10/g'
PIN=None
sha=lambda b:hashlib.sha256(b).hexdigest()

def main():
    global PIN
    parser=argparse.ArgumentParser();parser.add_argument('runtime_pin');parser.add_argument('--driver-revision');parser.add_argument('--index',action='store_true');args=parser.parse_args()
    PIN=args.runtime_pin;driver_revision=args.driver_revision or PIN
    assert re.fullmatch('[0-9a-f]{40}',PIN) and re.fullmatch('[0-9a-f]{40}',driver_revision)
    evidence=ROOT/'evidence'
    manifest=json.loads((ROOT/'manifest.json').read_text(encoding='utf8'))
    inventory=json.loads((evidence/'inventory.json').read_text(encoding='utf8'))
    assert manifest['format']=='spheres-s10g-increment/v1'
    assert manifest['runtime_revision']==manifest['logic_revision']==PIN and manifest['driver_revision']==driver_revision
    assert manifest['final_delta'] in ([],['tools/ui/ci-sanctions-desk.cjs'])
    delta=subprocess.check_output(['git','-c','core.longpaths=true','diff','--name-only',PIN,driver_revision],cwd=REPO,text=True).splitlines()
    assert delta==manifest['final_delta'],'Qualified browser-only corrections cannot change the runtime source'
    assert sha((evidence/'inventory.json').read_bytes())==manifest['evidence']['inventory_sha256']
    assert len(inventory['files'])==manifest['evidence']['logical_files']
    selection=BASE/'evidence/S10g-selection.json'
    assert sha(selection.read_bytes())==inventory['selection_sha256']
    assert json.loads(selection.read_text(encoding='utf8'))==inventory['selection']
    for file in [ROOT/'manifest.json',ROOT/'.gitattributes',evidence/'inventory.json',selection]:
        assert b'\r' not in file.read_bytes(),str(file)+' must use LF to retain index byte identity'
    names=set();logical=set();stored_bytes=0
    for row in inventory['files']:
        name=pathlib.PurePosixPath(row['logical_path'])
        assert not name.is_absolute() and '..' not in name.parts and str(name) not in logical
        logical.add(str(name));assert row['reconstruction_verified'] is True
        pieces=[]
        for part in row['storage']:
            name=pathlib.PurePosixPath(part['file']);assert not name.is_absolute() and '..' not in name.parts
            assert str(name) not in names;names.add(str(name));p=evidence/name
            assert p.is_file() and not p.is_symlink();raw=p.read_bytes()
            assert len(raw)==part['bytes'] and sha(raw)==part['sha256'];pieces.append(raw);stored_bytes+=len(raw)
        raw=b''.join(pieces)
        assert row['encoding'] in ('gzip','raw')
        if row['encoding']=='gzip':raw=gzip.decompress(raw)
        assert len(raw)==row['bytes'] and sha(raw)==row['sha256']
        source=pathlib.Path(row['source']);assert source.is_file() and not source.is_symlink()
        assert source.read_bytes()==raw,'Original selected source changed: '+str(source)
    assert stored_bytes==inventory['stored_bytes']==manifest['evidence']['stored_bytes']
    assert {p.relative_to(evidence).as_posix() for p in evidence.rglob('*') if p.is_file()}==names|{'inventory.json'}
    assert not any(p.is_symlink() for p in evidence.rglob('*'))
    if args.index:
        for name in names|{'inventory.json'}:
            rel='docs/campaign-certification/S10/g/evidence/'+name
            blob=subprocess.check_output(['git','-c','core.longpaths=true','show',':'+rel],cwd=REPO)
            assert blob==(evidence/name).read_bytes()
        for name in ['manifest.json','.gitattributes']:
            rel='docs/campaign-certification/S10/g/'+name
            blob=subprocess.check_output(['git','-c','core.longpaths=true','show',':'+rel],cwd=REPO)
            assert blob==(ROOT/name).read_bytes()
    for key in ['c01_complete','s10_complete','g2_earned','s11_started','historical_role_or_gameplay_authorization_changed']:assert manifest[key] is False
    assert manifest['browser_elapsed_days']==manifest['authored_browser_elapsed_days']==manifest['ordinary_browser_elapsed_days']==0
    q=manifest['qualification'];assert q['authored_cases']==1 and q['authored_commands']==3 and q['authored_previews']==4 and q['content_commands']==12
    assert q['authored_command_kinds']==['lift','sanction','improve'] and q['authored_envelope_comparisons']==9
    assert q['authored_browser']=={'inspections':13,'exact_comparisons':9,'ignored_paths':[]}
    assert q['ordinary_browser']['inspections']>0 and q['ordinary_browser']['exact_comparisons']>0 and q['ordinary_browser']['ignored_paths']==[]
    for lane in ['windows_web','windows_agency']:
        assert q[lane]['passed']>0 and q[lane]['failed']==0 and q[lane]['completed_targets']>0
    assert q['windows_node']['tests']==q['windows_node']['pass']+q['windows_node']['skipped'] and q['windows_node']['pass']>0 and q['windows_node']['fail']==0
    assert {c['name'] for c in q['linux_checks']}=={'web','agency'}
    assert all(c['totals']['passed']>0 and c['totals']['failed']==0 and c['totals']['completed_targets']>0 for c in q['linux_checks'])
    assert q['content_tests']==sum(q['content_test_groups'].values())==66
    assert set(q['content_test_groups'])=={'tonga-tests','brazil-tests','india-tests','japan-tests','census-tests','france-tests','portrait-tests'}
    assert q['new_long_campaign_claim'] is False and q['new_performance_claim'] is True
    assert len(q['performance_measurements'])==1
    measured=q['performance_measurements'][0]
    assert measured['passed'] and measured['samples']==len(measured['samples_ms'])==21 and measured['warmups']==3
    assert measured['source_file_unchanged'] and measured['benchmark_world_unchanged'] and measured['log_history_unchanged'] and measured['days_advanced']==0
    assert sorted(measured['samples_ms'])[19]==measured['p95_ms']<=measured['p95_limit_ms']==300
    assert max(measured['samples_ms'])==measured['max_ms']<=measured['max_limit_ms']==750
    for when in ['before','after']:
        assert measured['route_facts'][when]['enabled'] is True
        assert all(measured['route_facts'][when][direction]['available'] is True for direction in ['outbound','inbound'])
    reference=q['performance_source_reference'];assert reference['checkpoint']=='S08' and reference['reconstruction_verified']
    prior=REPO/reference['inventory_path'];assert sha(prior.read_bytes())==reference['inventory_sha256']
    assert json.loads(prior.read_text())['compressed_objects'][reference['source_sha256']]==reference['compressed_object']
    source=pathlib.Path(reference['source_path']);assert source.stat().st_size==reference['source_bytes'] and sha(source.read_bytes())==reference['source_sha256']
    print(json.dumps({'passed':True,'logical_files':len(inventory['files']),'stored_files':len(names)+1,'stored_bytes':stored_bytes,'index_verified':args.index,'runtime_revision':PIN,'driver_revision':driver_revision,'inventory_sha256':sha((evidence/'inventory.json').read_bytes())}))
if __name__=='__main__':main()
