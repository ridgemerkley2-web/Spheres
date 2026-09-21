"""Preserve selected S16 evidence, with byte-exact reconstructed large files."""
import argparse,datetime,gzip,hashlib,json,pathlib,sys
base=pathlib.Path(__file__).resolve().parent;repo=base/'integration'
def sha(p):
 with p.open('rb') as f:return hashlib.file_digest(f,'sha256').hexdigest()

def preflight(inputs,out):
 assert isinstance(inputs,list) and inputs,'Select at least one evidence source'
 assert not out.exists(),'Evidence destination already exists'
 plan=[];logical={'inventory.json'};storage={'inventory.json'}
 def reserve(name,seen):
  key=str(name).casefold();parts=pathlib.PurePosixPath(key).parts
  assert key not in seen,'Duplicate destination: '+str(name)
  assert not any('/'.join(parts[:i]) in seen for i in range(1,len(parts))),'Destination parent is a file: '+str(name)
  assert not any(old.startswith(key+'/') for old in seen),'Destination replaces a directory: '+str(name)
  # Reserve numbered piece names before writing, without compressing inputs
  # during preflight. A collision is refused even if it might fit one piece.
  assert not any(key.startswith(old+'.part-') or old.startswith(key+'.part-') for old in seen),'Chunk destination collision: '+str(name)
  seen.add(key)
 for item in inputs:
  supplied=pathlib.Path(item['source']);assert supplied.exists(),'Missing evidence source: '+str(supplied)
  assert not supplied.is_symlink(),'Evidence source must not be a symlink: '+str(supplied)
  source=supplied.resolve();prefix=pathlib.PurePosixPath(item['name'])
  assert str(prefix) not in ('','.') and not prefix.is_absolute() and '..' not in prefix.parts and chr(92) not in str(prefix) and ':' not in str(prefix),'Invalid logical destination'
  assert source.is_file() or source.is_dir(),'Evidence source is not a file or directory: '+str(source)
  entries=[source] if source.is_file() else sorted(source.rglob('*'))
  assert not any(p.is_symlink() for p in entries),'Evidence directory contains a symlink'
  files=[p for p in entries if p.is_file()];assert files,'Evidence directory is empty: '+str(source)
  for p in files:
   rel=prefix if source.is_file() else prefix/pathlib.PurePosixPath(p.relative_to(source).as_posix())
   reserve(rel,logical);reserve(str(rel)+('.gz' if p.stat().st_size>1024*1024 else ''),storage)
   plan.append((p,rel))
 return plan

def collect(selection,out):
 selection_raw=selection.read_bytes();inputs=json.loads(selection_raw.decode('utf-8-sig'))
 selection_hash=hashlib.sha256(selection_raw).hexdigest();plan=preflight(inputs,out)
 assert sha(selection)==selection_hash,'Evidence selection changed during preflight'
 assert out.parent.parent.is_dir() and not out.parent.is_symlink(),'Missing or linked certification directory'
 out.parent.mkdir(exist_ok=True)
 out.mkdir(exist_ok=False)
 records=[];chunk_limit=40*1024*1024
 for p,rel in plan:
  raw=p.read_bytes();digest=hashlib.sha256(raw).hexdigest();encoded=gzip.compress(raw,compresslevel=6,mtime=0) if len(raw)>1024*1024 else raw
  compressed=len(raw)>1024*1024;storage=[]
  for i,start in enumerate(range(0,len(encoded),chunk_limit)):
   suffix=('.gz' if compressed else '')+(f'.part-{i+1:03d}' if len(encoded)>chunk_limit else '')
   target=out/(str(rel)+suffix);target.parent.mkdir(parents=True,exist_ok=True)
   with target.open('xb') as f:f.write(encoded[start:start+chunk_limit])
   storage.append({'file':target.relative_to(out).as_posix(),'bytes':target.stat().st_size,'sha256':sha(target)})
  if not encoded:
   target=out/str(rel);target.parent.mkdir(parents=True,exist_ok=True);target.touch(exist_ok=False);storage=[{'file':target.relative_to(out).as_posix(),'bytes':0,'sha256':sha(target)}]
  reconstructed=b''.join((out/r['file']).read_bytes() for r in storage)
  if compressed:reconstructed=gzip.decompress(reconstructed)
  assert reconstructed==raw and sha(p)==digest
  records.append({'source':str(p),'logical_path':str(rel),'bytes':len(raw),'sha256':digest,'encoding':'gzip' if compressed else 'raw','storage':storage,'reconstruction_verified':True})
 assert sha(selection)==selection_hash,'Evidence selection changed during collection'
 manifest={'created_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'selection_sha256':selection_hash,'selection':inputs,'files':records,
  'scope':'Selected complete S16 test logs, proof records, browser outputs including failures, raw campaign snapshots and review launch. Compiled binaries are identified by SHA-256 in proofs and reproducible from the recorded source; they are not repackaged here.',
  'stored_bytes':sum(x['bytes'] for r in records for x in r['storage'])}
 with (out/'inventory.json').open('x',encoding='utf-8') as f:json.dump(manifest,f,indent=2);f.write('\n')
 print(json.dumps({'files':len(records),'stored_bytes':manifest['stored_bytes'],'inventory_sha256':sha(out/'inventory.json')}),flush=True)

if __name__=='__main__':
 parser=argparse.ArgumentParser(description='Retain complete selected S16 evidence. Default: read-only preflight. --apply creates only the new S16 evidence tree.')
 parser.add_argument('selection',type=pathlib.Path)
 parser.add_argument('--apply',action='store_true')
 args=parser.parse_args();selection=args.selection.resolve();out=repo/'docs/campaign-certification/S16/evidence'
 if args.apply:collect(selection,out)
 else:
  selected=json.loads(selection.read_text(encoding='utf-8-sig'));plan=preflight(selected,out)
  print(json.dumps({'passed':True,'applied':False,'selection_sha256':sha(selection),'destination':str(out),'files':len(plan),'source_bytes':sum(p.stat().st_size for p,_ in plan)},indent=2))
