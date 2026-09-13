"""Verify every stored byte, reconstruction, original source and optional Git index."""
import gzip,hashlib,json,pathlib,subprocess,sys
base=pathlib.Path(__file__).resolve().parent;repo=base/'integration';root=repo/'docs/campaign-certification/S10/d';evidence=root/'evidence'
sha=lambda b:hashlib.sha256(b).hexdigest()
manifest=json.loads((root/'manifest.json').read_text());inventory=json.loads((evidence/'inventory.json').read_text())
assert sha((evidence/'inventory.json').read_bytes())==manifest['evidence']['inventory_sha256']
assert len(inventory['files'])==manifest['evidence']['logical_files']
names=set()
for row in inventory['files']:
    pieces=[]
    for part in row['storage']:
        name=pathlib.PurePosixPath(part['file']);assert not name.is_absolute() and '..' not in name.parts
        assert str(name) not in names;names.add(str(name));raw=(evidence/name).read_bytes()
        assert len(raw)==part['bytes'] and sha(raw)==part['sha256'];pieces.append(raw)
    raw=b''.join(pieces)
    if row['encoding']=='gzip':raw=gzip.decompress(raw)
    assert len(raw)==row['bytes'] and sha(raw)==row['sha256']
    assert pathlib.Path(row['source']).read_bytes()==raw
assert {p.relative_to(evidence).as_posix() for p in evidence.rglob('*') if p.is_file()}==names|{'inventory.json'}
if '--index' in sys.argv:
    for name in names|{'inventory.json'}:
        rel='docs/campaign-certification/S10/d/evidence/'+name
        blob=subprocess.check_output(['git','-c','core.longpaths=true','show',':'+rel],cwd=repo)
        assert blob==(evidence/name).read_bytes()
for key in ['c01_complete','s10_complete','g2_earned','s11_started','historical_role_or_gameplay_authorization_changed']:assert manifest[key] is False
print(json.dumps({'passed':True,'logical_files':len(inventory['files']),'stored_files':len(names)+1,'index_verified':'--index' in sys.argv,'inventory_sha256':sha((evidence/'inventory.json').read_bytes())}))
