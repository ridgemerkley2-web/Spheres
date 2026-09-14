import hashlib,json,pathlib,subprocess,sys
BASE=pathlib.Path(__file__).resolve().parent;REPO=BASE/'integration';DEST=REPO/'docs/campaign-certification/S10/h';PIN=sys.argv[1];INDEX='--index' in sys.argv
def read(p):return json.loads(p.read_text(encoding='utf8'))
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
manifest=read(DEST/'manifest.json');inventory=read(DEST/'evidence/inventory.json')
assert manifest['source_revision']==PIN and manifest['evidence']['inventory_sha256']==sha(DEST/'evidence/inventory.json')
assert manifest['evidence']['files']==len(inventory['files'])
assert manifest['evidence']['stored_bytes']==inventory['stored_bytes']==sum(f['bytes'] for f in inventory['files'])
for field in ['game_runtime_changed','historical_roles_or_gameplay_grants_changed','s10_complete','c01_complete','g2_earned','s11_started']:assert manifest[field] is False
paths=set()
for row in inventory['files']:
 path=pathlib.PurePosixPath(row['path']);assert not path.is_absolute() and '..' not in path.parts and row['path'] not in paths;paths.add(row['path'])
 file=DEST/'evidence'/path;assert file.is_file() and not file.is_symlink() and file.stat().st_size==row['bytes'] and sha(file)==row['sha256']
 assert file.read_bytes()==pathlib.Path(row['source']).read_bytes()
assert {p.relative_to(DEST/'evidence').as_posix() for p in (DEST/'evidence').rglob('*') if p.is_file()}==paths|{'inventory.json'}
for file in [DEST/'manifest.json',DEST/'.gitattributes',DEST/'evidence/inventory.json']:assert b'\r' not in file.read_bytes()
if INDEX:
 for rel in ['manifest.json','.gitattributes',*['evidence/'+p for p in paths|{'inventory.json'}]]:
  staged=subprocess.check_output(['git','-c','core.longpaths=true','show',':docs/campaign-certification/S10/h/'+rel],cwd=REPO);assert staged==(DEST/rel).read_bytes()
print(json.dumps({'passed':True,'source_revision':PIN,'index_verified':INDEX,'files':len(paths),'stored_bytes':inventory['stored_bytes']}))
