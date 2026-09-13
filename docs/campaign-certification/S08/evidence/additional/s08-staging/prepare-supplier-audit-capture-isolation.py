"""Prepare exact outside test-harness capture isolation atop cleanup-order fix."""
from pathlib import Path
import hashlib
import difflib
import json
staging=Path(__file__).resolve().parent
original=staging.parent/'integration/tools/ui/ci-supplier-imports.cjs'
prior=staging/'ci-supplier-imports.cleanup-order.cjs'
sha=lambda data:hashlib.sha256(data).hexdigest()
assert sha(original.read_bytes())=='de8a13611ce1617133710b252253cd82e41d6a4b74d39799f7d29eb2b97ce7f1'
assert sha(prior.read_bytes())=='c74f8a8d1f1ee090113179a5a34acfada0ffa28e8689de4e3aba8fa766e21b9c'
old="""async function archive(page,url,run,slot){
  const response=await page.request.post(url+'/api/save',{data:{slot}});
  try{assert(response.ok(),'Could not capture '+slot);await response.body();}finally{await response.dispose();}
  const buyer=await page.evaluate(()=>S.player);assert(buyer,'Snapshot must identify the actual player');
  return audit.inspect(path.join(run,'saves',slot+'.json'),buyer);
}
"""
new="""async function archive(page,url,run,slot){
  // These are evidence captures only. Keep real input/named campaign slots in
  // saves, while preserving audit bytes outside the player's save-list scan.
  const auditSlots=new Set(['s08-preparation-before','s08-preparation-after',
    's08-preview-before','s08-preview-after','s08-purchase-before','s08-purchase-after',
    's08-cancel-review','s08-paid-transit','s08-import-midpoint-before','s08-import-midpoint-after',
    's08-delivered','s08-maintenance-review','s08-maintenance-approved',
    's08-import-complete-before','s08-import-complete-after','s08-continue',
    ...Array.from({length:7},(_,i)=>'s08-maintenance-day-'+(i+1))]);
  assert(auditSlots.has(slot),'Refuse to relocate a non-audit campaign slot: '+slot);
  const root=fs.realpathSync(run),saves=path.join(root,'saves'),source=path.join(saves,slot+'.json');
  const same=(a,b)=>path.relative(a,b)==='';
  const inside=(parent,child)=>{const relative=path.relative(parent,child);assert(relative&&!relative.startsWith('..'+path.sep)&&relative!=='..'&&!path.isAbsolute(relative),'Audit path escapes its disposable directory');};
  inside(root,saves);inside(saves,source);
  assert(same(fs.realpathSync(saves),saves),'Audit saves directory must remain inside this disposable run');
  assert(!fs.existsSync(source),'Refuse to overwrite an existing audit-named campaign: '+source);
  const response=await page.request.post(url+'/api/save',{data:{slot}});let capture;
  try{
    assert(response.ok(),'Could not capture '+slot);await response.body();
    assert(fs.lstatSync(source).isFile()&&!fs.lstatSync(source).isSymbolicLink(),'Native audit capture must be an ordinary file');
    assert(same(fs.realpathSync(source),source),'Native capture must remain in the disposable saves directory');
    const captures=path.join(root,'audit-captures');inside(root,captures);
    if(!fs.existsSync(captures))fs.mkdirSync(captures);
    assert(fs.lstatSync(captures).isDirectory()&&!fs.lstatSync(captures).isSymbolicLink()&&same(fs.realpathSync(captures),captures),'Audit target must remain in this disposable run');
    // The OS creates a new exclusive directory; never replace prior evidence.
    const directory=fs.mkdtempSync(path.join(captures,slot+'-'));inside(captures,directory);
    assert(same(fs.realpathSync(directory),directory),'Exclusive audit directory escaped its parent');
    capture=path.join(directory,slot+'.json');inside(directory,capture);
    assert(!fs.existsSync(capture),'Refuse to overwrite preserved audit evidence');
    fs.renameSync(source,capture);
  }finally{await response.dispose();}
  const buyer=await page.evaluate(()=>S.player);assert(buyer,'Snapshot must identify the actual player');
  return audit.inspect(capture,buyer);
}
"""
text=prior.read_bytes().decode('utf-8')
if text.count(old)!=1:old=old.replace('\n','\r\n');new=new.replace('\n','\r\n')
assert text.count(old)==1
candidate=text.replace(old,new,1)
target=staging/'ci-supplier-imports.audit-capture-isolation.cjs'
target.write_bytes(candidate.encode('utf-8'))
def patch(a,b,name):
 p=staging/name
 p.write_bytes(''.join(difflib.unified_diff(a.replace('\r\n','\n').splitlines(True),b.replace('\r\n','\n').splitlines(True),fromfile='a/tools/ui/ci-supplier-imports.cjs',tofile='b/tools/ui/ci-supplier-imports.cjs')).encode('utf-8'))
 return {'path':str(p),'sha256':sha(p.read_bytes())}
record={'source':str(original),'source_sha256':sha(original.read_bytes()),'prior':str(prior),'prior_sha256':sha(prior.read_bytes()),'candidate':str(target),'candidate_sha256':sha(target.read_bytes()),'incremental_patch':patch(text,candidate,'supplier-audit-capture-isolation.incremental.patch'),'complete_patch':patch(original.read_bytes().decode('utf-8'),candidate,'supplier-audit-capture-isolation.patch'),'scope':'Two reviewed harness blocks only:cleanup-order and audit-only capture placement. Original runtime/assets and real save/load controls unchanged.'}
(staging/'supplier-audit-capture-isolation-bindings.json').write_text(json.dumps(record,indent=2)+'\n',encoding='utf-8')
print(json.dumps(record,indent=2))
