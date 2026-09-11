import pathlib,subprocess,json,hashlib,sys
repo=pathlib.Path.cwd();session=repo/'docs/campaign-certification/S07'
manifest=json.loads((session/'manifest.json').read_text(encoding='utf-8'))
rows=manifest['evidence_files'];assert rows
paths=[str(pathlib.PurePosixPath('docs/campaign-certification/S07')/r['file']) for r in rows]
data=subprocess.check_output(['git','cat-file','--batch'],input=''.join(':'+p+'\n' for p in paths).encode(),cwd=repo)
at=0;bad=[]
for p,r in zip(paths,rows):
 end=data.index(b'\n',at);head=data[at:end].split();assert len(head)==3 and head[1]==b'blob',(p,head)
 size=int(head[2]);blob=data[end+1:end+1+size];at=end+size+2
 digest=hashlib.sha256(blob).hexdigest()
 if size!=r['bytes'] or digest!=r['sha256']:bad.append(dict(path=p,actual_bytes=size,actual_sha256=digest,expected=r))
assert at==len(data)
result=dict(runtime_revision=manifest['runtime_revision'],index_evidence_files=len(rows),mismatches=bad,passed=not bad)
(repo.parent/'S07-staged-evidence-audit.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps(result,indent=2));assert not bad
