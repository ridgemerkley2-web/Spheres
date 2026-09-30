"""Read-only verified GitHub ZIP -> fresh owned frozen/; never executes native code."""
import hashlib,importlib,json,os,pathlib,stat,sys,zipfile
BASE=pathlib.Path(__file__).resolve().parent
ARCHIVE=BASE/"full-stability-frozen-1.zip"
EXPECTED_ZIP="c0b8cac48eb04e5c4bf171d44ae480bcc940e7884c236ab973963fed4621021f"
EXPECTED_MANIFEST="70a51be54a7e3100a5267ca310bf5c7f05a3c388a389b7c54904e6365c477cc8"
NAMES={"manifest.json","native-test","plan.json","stability_matrix.py","distributed_stability.py"}
def pin(path):
 h=hashlib.sha256()
 with path.open("rb") as f:
  while b:=f.read(1048576):h.update(b)
 return {"bytes":path.stat().st_size,"sha256":h.hexdigest()}
before=pin(ARCHIVE)
assert before["sha256"]==EXPECTED_ZIP
out=BASE/"frozen"
assert not out.exists() and not out.is_symlink(),"Refuse overwrite of extracted directory"
with zipfile.ZipFile(ARCHIVE) as z:
 infos=z.infolist()
 assert len(infos)==len(NAMES) and {i.filename for i in infos}==NAMES
 for i in infos:
  assert "/" not in i.filename and "\\" not in i.filename and ":" not in i.filename
  assert not i.is_dir() and not stat.S_ISLNK(i.external_attr>>16)
  assert stat.S_IFMT(i.external_attr>>16) in (0,stat.S_IFREG)
 raw=z.read("manifest.json")
 assert hashlib.sha256(raw).hexdigest()==EXPECTED_MANIFEST
 manifest=json.loads(raw)
 assert set(manifest["files"])==NAMES-{"manifest.json"}
 assert manifest["revision"]=="5d970f6d7370baf16760585c641d81253d1c2175"
 assert manifest["workflow_run_id"]=="36474011141" and manifest["workflow_run_attempt"]=="1"
 expected={**manifest["files"],"manifest.json":{"bytes":len(raw),"sha256":EXPECTED_MANIFEST}}
 for i in infos:
  assert i.file_size==expected[i.filename]["bytes"]
 out.mkdir()
 details={}
 for i in infos:
  dst=out/i.filename
  assert dst.parent.resolve()==out.resolve()
  h=hashlib.sha256(); n=0
  with z.open(i) as src,dst.open("xb") as sink:
   while b:=src.read(1048576):sink.write(b);h.update(b);n+=len(b)
   sink.flush();os.fsync(sink.fileno())
  details[i.filename]={"bytes":n,"sha256":h.hexdigest()}
  assert details[i.filename]==expected[i.filename]
  assert pin(dst)==expected[i.filename]
assert pin(ARCHIVE)==before
sys.path.insert(0,str(out))
driver=importlib.import_module("distributed_stability")
root,checked,plan=driver.bundle(out,EXPECTED_MANIFEST)
assert len(plan["cells"])==24
report={"format":"spheres-distributed-extraction/v1","passed":True,"source_archive":str(ARCHIVE),"source_archive_pin":before,"manifest_sha256":EXPECTED_MANIFEST,"revision":checked["revision"],"run_id":checked["workflow_run_id"],"attempt":checked["workflow_run_attempt"],"extracted_root":str(root),"files_verified":details,"frozen_driver_bundle_validation":True,"full_plan_cells":len(plan["cells"]),"native_executed":False,"full_matrix_passed":False,"qualification":False}
with (BASE/"extraction-report.json").open("x",encoding="utf-8") as f:json.dump(report,f,indent=2);f.write("\n")
print(json.dumps({k:v for k,v in report.items() if k!="files_verified"},indent=2))

