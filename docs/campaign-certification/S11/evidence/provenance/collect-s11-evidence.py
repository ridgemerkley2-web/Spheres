"""Retain complete selected S11 evidence, including failed attempts."""
import importlib.util,json,pathlib,sys
base=pathlib.Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location('retention',base/'collect-s09-evidence.py')
module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module)
out=base/'integration/docs/campaign-certification/S11/evidence'
module.collect(pathlib.Path(sys.argv[1]).resolve(),out)
p=out/'inventory.json';data=json.loads(p.read_text(encoding='utf8'))
data['scope']='Selected complete S11 logs and proof records, authored native fixtures, every retained browser stage (including failed attempts), full saved worlds and review launch. Large files are compressed without omitting native fields. Compiled binaries are identified by SHA-256 and reproduced from the source rather than repackaged.'
p.write_text(json.dumps(data,indent=2)+'\n',encoding='utf8')
print(json.dumps({'inventory_sha256':module.sha(p),'files':len(data['files']),'stored_bytes':data['stored_bytes']}))
