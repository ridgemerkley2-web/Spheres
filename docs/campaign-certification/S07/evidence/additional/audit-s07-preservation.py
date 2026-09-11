import json,pathlib,hashlib,subprocess,datetime
base=pathlib.Path.cwd().parent
repo=base/'integration'
pin='041007fbfda48cc027d0c24a1189705a1dcaa89b'
def sha(p):return hashlib.file_digest(p.open('rb'),'sha256').hexdigest()
def git(p,*args):return subprocess.check_output(['git',*args],cwd=p,text=True).strip()
prior=json.loads((base/'evidence/S07-preservation-before.json').read_text())
rows=[]
for old in prior['files']:
 p=pathlib.Path(old['path']);size=p.stat().st_size;h=sha(p)
 rows.append(dict(path=str(p),bytes=size,sha256=h,unchanged=size==old['bytes'] and h==old['sha256']))
worktrees=[]
for p,expected in [(base.parent/'Spheres','038fe0b1a3edb1772586ec0b5a7451629b7297c4'),(base/'master-baseline','485c223f60d5ff6e46f6ae17164bf1ee3a8764d9')]:
 head=git(p,'rev-parse','HEAD');status=git(p,'status','--porcelain')
 worktrees.append(dict(path=str(p),head=head,status=status,unchanged=head==expected and not status))
linux=json.loads((base/'evidence/S07-linux-final/runner-result.json').read_text())
fixtures=[]
for old in linux['fixtures_after']['items']:
 p=pathlib.Path('C:/'+old['path'].removeprefix('/mnt/c/'));h=sha(p)
 fixtures.append(dict(path=str(p),sha256=h,unchanged=h==old['sha256']))
result=dict(candidate=pin,captured_utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),files=rows,worktrees=worktrees,external_fixtures=fixtures)
result['passed']=all(r['unchanged'] for r in rows+worktrees+fixtures)
(base/'evidence/S07-preservation.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps(dict(passed=result['passed'],protected=len(rows),worktrees=worktrees,external_fixture_count=len(fixtures)),indent=2))
assert result['passed']

