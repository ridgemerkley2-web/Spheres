"""Verify each S10.f stored byte, reconstruction, source and optional Git index."""
import argparse,gzip,hashlib,json,pathlib,re,subprocess
BASE=pathlib.Path(__file__).resolve().parent
REPO=BASE/'integration'
ROOT=REPO/'docs/campaign-certification/S10/f'
PIN='7d85b9f76a3e787b257613442fb0a3370e90f4c2'
sha=lambda b:hashlib.sha256(b).hexdigest()

def main():
    parser=argparse.ArgumentParser();parser.add_argument('driver_revision');parser.add_argument('--index',action='store_true');args=parser.parse_args()
    assert re.fullmatch('[0-9a-f]{40}',args.driver_revision)
    evidence=ROOT/'evidence'
    manifest=json.loads((ROOT/'manifest.json').read_text(encoding='utf8'))
    inventory=json.loads((evidence/'inventory.json').read_text(encoding='utf8'))
    assert manifest['format']=='spheres-s10f-increment/v1'
    assert manifest['runtime_revision']==manifest['logic_revision']==PIN and manifest['driver_revision']==args.driver_revision
    assert manifest['final_delta']==['tools/ui/ci-succession-context.cjs']
    delta=subprocess.check_output(['git','-c','core.longpaths=true','diff','--name-only',PIN,args.driver_revision],cwd=REPO,text=True).splitlines()
    assert delta==manifest['final_delta'],'The qualified driver differs from the runtime pin only in the visibility assertion file'
    assert sha((evidence/'inventory.json').read_bytes())==manifest['evidence']['inventory_sha256']
    assert len(inventory['files'])==manifest['evidence']['logical_files']
    selection=BASE/'evidence/S10f-selection.json'
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
            rel='docs/campaign-certification/S10/f/evidence/'+name
            blob=subprocess.check_output(['git','-c','core.longpaths=true','show',':'+rel],cwd=REPO)
            assert blob==(evidence/name).read_bytes()
        for name in ['manifest.json','.gitattributes']:
            rel='docs/campaign-certification/S10/f/'+name
            blob=subprocess.check_output(['git','-c','core.longpaths=true','show',':'+rel],cwd=REPO)
            assert blob==(ROOT/name).read_bytes()
    for key in ['c01_complete','s10_complete','g2_earned','s11_started','historical_role_or_gameplay_authorization_changed']:assert manifest[key] is False
    assert manifest['browser_elapsed_days']==manifest['authored_browser_elapsed_days']==manifest['ordinary_browser_elapsed_days']==0
    q=manifest['qualification'];assert q['authored_cases']==2 and q['authored_commands']==q['authored_previews']==0 and q['content_commands']==11
    assert q['authored_case_names']==['uk-1990-same-day-death','uk-2027-term-limit']
    assert q['authored_browser']=={'inspections':12,'exact_comparisons':10,'ignored_paths':[]}
    assert q['ordinary_browser']['inspections']>0 and q['ordinary_browser']['exact_comparisons']>0 and q['ordinary_browser']['ignored_paths']==[]
    for lane in ['windows_web','windows_leadership','windows_agency']:
        assert q[lane]['passed']>0 and q[lane]['failed']==0 and q[lane]['completed_targets']>0
    assert q['windows_node']['tests']==q['windows_node']['pass']+q['windows_node']['skipped'] and q['windows_node']['pass']>0 and q['windows_node']['fail']==0
    assert {c['name'] for c in q['linux_checks']}=={'web','leadership','agency'}
    assert all(c['totals']['passed']>0 and c['totals']['failed']==0 and c['totals']['completed_targets']>0 for c in q['linux_checks'])
    assert q['content_tests']==sum(q['content_test_groups'].values())
    assert set(q['content_test_groups'])=={'brazil-tests','india-tests','japan-tests','census-tests','france-tests','portrait-tests'}
    assert q['new_long_campaign_or_performance_claim'] is False
    print(json.dumps({'passed':True,'logical_files':len(inventory['files']),'stored_files':len(names)+1,'stored_bytes':stored_bytes,'index_verified':args.index,'runtime_revision':PIN,'driver_revision':args.driver_revision,'inventory_sha256':sha((evidence/'inventory.json').read_bytes())}))
if __name__=='__main__':main()
