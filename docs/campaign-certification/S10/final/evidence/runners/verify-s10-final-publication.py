"""Verify complete retained evidence, reconstruction, scope and optional Git index bytes."""
import gzip, hashlib, json, pathlib, subprocess, sys
BASE=pathlib.Path(__file__).resolve().parent
REPO=BASE/'integration'
DEST=REPO/'docs/campaign-certification/S10/final'
PIN='bcdf72bcbe8947966359deb95f715cb526a68def'
INDEX='--index' in sys.argv
def read(p): return json.loads(p.read_text(encoding='utf-8-sig'))
def sha(b): return hashlib.sha256(b).hexdigest()
manifest=read(DEST/'manifest.json')
inventory=read(DEST/'evidence/inventory.json')
assert manifest['format']=='spheres-s10-final/v1'
assert manifest['candidate_revision']==PIN and manifest['qualification']['passed']
assert manifest['c01_complete'] is False and manifest['s11_started'] is False
if manifest['s10_complete']:
    assert manifest['status']=='s10_gameplay_complete' and manifest['g2_earned'] is True
    assert manifest['approval']['answer']=='Finish S10 gameplay; keep content certification separate (Recommended)'
    assert manifest['approval']['received_utc'] and manifest['approval']['question']
else:
    assert manifest['status']=='gameplay_qualified_scope_pending' and manifest['g2_earned'] is False
assert manifest['evidence']['inventory_sha256']==sha((DEST/'evidence/inventory.json').read_bytes())
assert manifest['evidence']['logical_files']==len(inventory['files'])
logical=set(); storage=set(); stored_bytes=0
for row in inventory['files']:
    name=pathlib.PurePosixPath(row['logical_path'])
    assert not name.is_absolute() and '..' not in name.parts and str(name) not in logical
    logical.add(str(name)); pieces=[]
    for piece in row['storage']:
        rel=pathlib.PurePosixPath(piece['file'])
        assert not rel.is_absolute() and '..' not in rel.parts and str(rel) not in storage
        storage.add(str(rel)); file=DEST/'evidence'/rel
        assert file.is_file() and not file.is_symlink()
        raw=file.read_bytes()
        assert len(raw)==piece['bytes'] and sha(raw)==piece['sha256']
        stored_bytes+=len(raw); pieces.append(raw)
    raw=b''.join(pieces)
    assert row['encoding'] in ('gzip','raw')
    if row['encoding']=='gzip': raw=gzip.decompress(raw)
    assert len(raw)==row['bytes'] and sha(raw)==row['sha256'] and row['reconstruction_verified']
    assert raw==pathlib.Path(row['source']).read_bytes()
assert stored_bytes==inventory['stored_bytes']==manifest['evidence']['stored_bytes']
assert {p.relative_to(DEST/'evidence').as_posix() for p in (DEST/'evidence').rglob('*') if p.is_file()}==storage|{'inventory.json'}
for rel in ('manifest.json','.gitattributes','evidence/inventory.json'):
    assert b'\r' not in (DEST/rel).read_bytes()
if INDEX:
    for file in sorted(p for p in DEST.rglob('*') if p.is_file()):
        rel=file.relative_to(REPO).as_posix()
        staged=subprocess.check_output(['git','-c','core.longpaths=true','show',':'+rel],cwd=REPO)
        assert staged==file.read_bytes(),rel
print(json.dumps({'passed':True,'candidate_revision':PIN,'index_verified':INDEX,'files':len(logical),'stored_bytes':stored_bytes,'s10_complete':manifest['s10_complete']}))
