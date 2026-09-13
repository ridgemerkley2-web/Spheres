"""Reconstruct all selected S10.b evidence and verify exact Git index coverage."""
import gzip,hashlib,json,pathlib,subprocess
base=pathlib.Path(__file__).resolve().parent;repo=base/'integration'
root=repo/'docs/campaign-certification/S10/b';evidence=root/'evidence'
sha=lambda b:hashlib.sha256(b).hexdigest()
load=lambda p:json.loads(p.read_text(encoding='utf-8'))
manifest=load(root/'manifest.json');inventory=load(evidence/'inventory.json')
assert manifest['evidence']['inventory_sha256']==sha((evidence/'inventory.json').read_bytes())
storage=set()
for row in inventory['files']:
    pieces=[]
    for item in row['storage']:
        path=evidence/item['file'];raw=path.read_bytes()
        assert len(raw)==item['bytes'] and sha(raw)==item['sha256'];pieces.append(raw)
        assert item['file'] not in storage;storage.add(item['file'])
    raw=b''.join(pieces)
    if row['encoding']=='gzip':raw=gzip.decompress(raw)
    assert len(raw)==row['bytes'] and sha(raw)==row['sha256']
storage.add('inventory.json')
git=lambda *a:subprocess.check_output(['git','-c','core.longpaths=true',*a],cwd=repo)
index=git('ls-files','-s','--','docs/campaign-certification/S10/b/evidence').decode().splitlines()
files={line.split('\t',1)[1]:line.split()[1] for line in index}
expected={str((evidence/p).relative_to(repo).as_posix()) for p in storage}
assert set(files)==expected,'An evidence path is absent from Git or extra files were staged'
for name,object_id in files.items():
    raw=(repo/name).read_bytes();actual=hashlib.sha1(b'blob '+str(len(raw)).encode()+b'\0'+raw).hexdigest()
    assert actual==object_id,'Git index does not preserve the evidence bytes: '+name
runtime=manifest['runtime_revision']
assert not git('diff','--name-only',runtime,'HEAD','--','spheres-sim','spheres-cli','spheres-web','Cargo.toml','Cargo.lock').strip()
assert not git('diff','--name-only','HEAD','--','spheres-sim','spheres-cli','spheres-web','Cargo.toml','Cargo.lock').strip()
pathway=load(repo/'docs/planning/campaign-pathway.json')
assert next(s for s in pathway['sessions'] if s['id']=='S10')['status']=='in_progress'
assert next(s for s in pathway['sessions'] if s['id']=='C01')['status']=='in_progress'
assert not manifest['s10_complete'] and not manifest['c01_complete'] and not manifest['g2_earned']
assert not manifest['s11_started'] and pathway['last_completed_session']=='S09'
print(json.dumps({'passed':True,'logical_files':len(inventory['files']),'git_blobs':len(files),
                  'stored_bytes':inventory['stored_bytes'],'inventory_sha256':sha((evidence/'inventory.json').read_bytes())}))
