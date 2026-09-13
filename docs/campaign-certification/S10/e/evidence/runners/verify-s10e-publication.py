"""Verify each S10.e stored byte, reconstruction, source and optional Git index."""
import gzip,hashlib,json,pathlib,subprocess,sys
BASE=pathlib.Path(__file__).resolve().parent
REPO=BASE/'integration'
ROOT=REPO/'docs/campaign-certification/S10/e'
PIN='99b0b9d4a46a98a2c0cf7886583fe13e568dda64'
sha=lambda b:hashlib.sha256(b).hexdigest()

def main():
    evidence=ROOT/'evidence'
    manifest=json.loads((ROOT/'manifest.json').read_text(encoding='utf8'))
    inventory=json.loads((evidence/'inventory.json').read_text(encoding='utf8'))
    assert manifest['format']=='spheres-s10e-increment/v1'
    assert manifest['runtime_revision']==manifest['logic_revision']==PIN and manifest['final_delta']==[]
    assert sha((evidence/'inventory.json').read_bytes())==manifest['evidence']['inventory_sha256']
    assert len(inventory['files'])==manifest['evidence']['logical_files']
    selection=BASE/'evidence/S10e-selection.json'
    assert sha(selection.read_bytes())==inventory['selection_sha256']
    assert json.loads(selection.read_text(encoding='utf8'))==inventory['selection']
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
    if '--index' in sys.argv:
        for name in names|{'inventory.json'}:
            rel='docs/campaign-certification/S10/e/evidence/'+name
            blob=subprocess.check_output(['git','-c','core.longpaths=true','show',':'+rel],cwd=REPO)
            assert blob==(evidence/name).read_bytes()
        for name in ['manifest.json','.gitattributes']:
            rel='docs/campaign-certification/S10/e/'+name
            blob=subprocess.check_output(['git','-c','core.longpaths=true','show',':'+rel],cwd=REPO)
            assert blob==(ROOT/name).read_bytes()
    for key in ['c01_complete','s10_complete','g2_earned','s11_started','historical_role_or_gameplay_authorization_changed']:assert manifest[key] is False
    assert manifest['browser_elapsed_days']==manifest['authored_browser_elapsed_days']==1 and manifest['ordinary_browser_elapsed_days']==0
    q=manifest['qualification'];assert q['cancelled_choices']==2 and q['confirmed_replies']==1 and q['content_commands']==10
    assert q['content_tests']==sum(q['content_test_groups'].values())
    assert q['new_long_campaign_or_performance_claim'] is False
    print(json.dumps({'passed':True,'logical_files':len(inventory['files']),'stored_files':len(names)+1,'stored_bytes':stored_bytes,'index_verified':'--index' in sys.argv,'inventory_sha256':sha((evidence/'inventory.json').read_bytes())}))
if __name__=='__main__':main()
