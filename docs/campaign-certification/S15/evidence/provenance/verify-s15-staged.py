"""Verify staged S15 evidence preserves every published byte."""
import hashlib,json,pathlib,subprocess,sys
base=pathlib.Path(__file__).resolve().parent;repo=base/'integration';root=repo/'docs/campaign-certification/S15'
output=pathlib.Path(sys.argv[1]).resolve();assert not output.exists()
inventory=json.loads((root/'evidence/inventory.json').read_text(encoding='utf8'))
expected={'evidence/'+part['file']:part for row in inventory['files'] for part in row['storage']}
assert len(expected)==sum(len(row['storage']) for row in inventory['files'])
for relative,part in expected.items():
    raw=(root/relative).read_bytes()
    assert len(raw)==part['bytes'] and hashlib.sha256(raw).hexdigest()==part['sha256'],relative
actual={p.relative_to(root).as_posix() for p in (root/'evidence').rglob('*') if p.is_file()}
assert actual==set(expected)|{'evidence/inventory.json'}
result=subprocess.check_output(['git','-c','core.longpaths=true','ls-files','--stage','-z','--','docs/campaign-certification/S15'],cwd=repo)
seen=set()
for item in result.split(b'\0'):
    if not item:continue
    info,relative=item.split(b'\t',1);mode,oid,stage=info.split();assert stage==b'0' and mode==b'100644'
    relative=relative.decode('utf8');raw=(repo/relative).read_bytes()
    digest=hashlib.sha1(b'blob '+str(len(raw)).encode()+b'\0'+raw).hexdigest()
    assert digest==oid.decode(), 'Staged bytes differ: '+relative
    seen.add(relative)
assert seen=={p.relative_to(repo).as_posix() for p in root.rglob('*') if p.is_file()}
record={'passed':True,'staged_files':len(seen),'retained_source_files':len(inventory['files']),
    'evidence_storage_files':len(expected),'inventory_sha256':hashlib.sha256((root/'evidence/inventory.json').read_bytes()).hexdigest(),
    'scope':'Every S15 publication file matches its staged Git blob. Every retained storage file also matches the evidence inventory; no unlisted storage files exist.'}
output.write_text(json.dumps(record,indent=2)+'\n',encoding='utf8');print(json.dumps(record),flush=True)

