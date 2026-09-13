import datetime,hashlib,importlib.util,json,pathlib,sys
sys.dont_write_bytecode=True
out=pathlib.Path(__file__).resolve().parent;base=out.parents[1]
def module(name,file):
 spec=importlib.util.spec_from_file_location(name,file);m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m);return m
collector=module('s09_collector',base/'collect-s09-evidence.py');launch=module('s09_launch',base/'run-s09-launch-review.py')
fixtures=out/'fixtures';fixtures.mkdir();small=fixtures/'small';small.write_bytes(b'proof');zero=fixtures/'zero';zero.touch();large=fixtures/'large';large.write_bytes(b'x'*(1024*1024+1));empty=fixtures/'empty';empty.mkdir()
checks=[]
def one(p,name):return {'source':str(p),'name':name}
def preflight(label,inputs,valid):
 destination=fixtures/('output-'+label)
 try:plan=collector.preflight(inputs,destination);passed=True
 except AssertionError as e:plan=[];passed=False;reason=str(e)
 assert passed==valid,label
 assert not destination.exists(),label+' created output'
 checks.append({'name':label,'expected_valid':valid,'passed':True,'planned_files':len(plan),**({} if passed else {'refusal':reason})})
preflight('valid-files',[one(small,'proof.json'),one(zero,'empty.log')],True)
preflight('nonempty-dir',[one(fixtures,'fixture-copy')],True)
preflight('missing',[one(fixtures/'absent','missing')],False)
preflight('empty-selection',[],False)
preflight('empty-dir',[one(empty,'empty')],False)
preflight('duplicate',[one(small,'proof'),one(zero,'proof')],False)
preflight('windows-case',[one(small,'Proof'),one(zero,'proof')],False)
preflight('parent-file',[one(small,'proof'),one(zero,'proof/child')],False)
preflight('child-file',[one(small,'proof/child'),one(zero,'proof')],False)
preflight('reserved-inventory',[one(small,'inventory.json')],False)
preflight('gzip-collision',[one(large,'archive'),one(small,'archive.gz')],False)
preflight('chunk-collision',[one(large,'archive'),one(small,'archive.gz.part-001')],False)
preflight('traversal',[one(small,'../outside')],False)
a=fixtures/'canonical-a';b=fixtures/'canonical-b';a.write_bytes(b'a'*(1024*1024)+b'end');b.write_bytes(a.read_bytes());assert launch.same_bytes(a,b)
checks.append({'name':'canonical-equal-across-chunk','passed':True})
b.write_bytes(b'a'*(1024*1024)+b'bad');assert not launch.same_bytes(a,b);checks.append({'name':'canonical-late-byte-mismatch','passed':True})
b.write_bytes(a.read_bytes()+b'extra');assert not launch.same_bytes(a,b);checks.append({'name':'canonical-length-mismatch','passed':True})
files=[base/'collect-s09-evidence.py',base/'run-s09-launch-review.py',pathlib.Path(__file__)]
result={'passed':True,'scope':'Pure collector preflight and streaming byte-comparison helpers only. Collector, native archive worker, game and launcher were not executed. No repository writes.','checks':checks,'sources':[{'path':str(p),'sha256':hashlib.sha256(p.read_bytes()).hexdigest()} for p in files],'utc':datetime.datetime.now(datetime.timezone.utc).isoformat()}
(out/'result.json').write_text(json.dumps(result,indent=2)+'\n',encoding='utf-8');print(json.dumps({'passed':True,'checks':len(checks),'result':str(out/'result.json')}))
