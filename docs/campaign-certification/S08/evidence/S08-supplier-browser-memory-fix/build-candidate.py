"""Create the external harness candidate; do not run a browser or edit the repo."""
import difflib, hashlib, json, pathlib, re, subprocess
base = pathlib.Path(__file__).resolve().parent
original = base.parent / 'S08-supplier-browser-harness-fix/ci-supplier-imports.cjs'
repo = base.parent.parent / 'integration'
source = original.read_text(encoding='utf-8')
text = source

def replace(before, after):
    global text
    assert text.count(before) == 1, (text.count(before), before[:130])
    text = text.replace(before, after)

replace("const copy=v=>JSON.parse(JSON.stringify(v));", "const audit=require(process.env.SPHERES_SUPPLIER_ARCHIVE_AUDIT||'./supplier-archive-audit.cjs');\nconst copy=v=>JSON.parse(JSON.stringify(v));")
replace("async function read(page,url,p){const r=await page.request.get(url+p);assert(r.ok(),p+': '+r.status());return r.json();}",
        "async function read(page,url,p){const r=await page.request.get(url+p);try{assert(r.ok(),p+': '+r.status());return await r.json();}finally{await r.dispose();}}")
start = text.index('function worldOf(')
end = text.index('async function idle(', start)
text = text[:start] + """function contract(w,id){const d=w.imports.matching.find(d=>d.id===id);assert(d,'Missing import contract '+id);return d;}
function held(w,buyer,revision){assert.equal(w.buyer.id,buyer);return w.buyer.held_by_revision[revision]||0;}
function sourceProduct(w,offer){const c=w.supplier;assert(c);assert.equal(c.id,offer.company);assert.equal(c.nation,offer.seller_nation);assert.equal(c.product?.id,offer.product);return c.product;}
""" + text[end:]
replace("""  const response=await page.request.post(url+'/api/save',{data:{slot}});assert(response.ok(),'Could not capture '+slot);
  return worldOf(JSON.parse(fs.readFileSync(path.join(run,'saves',slot+'.json'),'utf8')));""",
"""  const response=await page.request.post(url+'/api/save',{data:{slot}});
  try{assert(response.ok(),'Could not capture '+slot);await response.body();}finally{await response.dispose();}
  const buyer=await page.evaluate(()=>S.player);assert(buyer,'Snapshot must identify the actual player');
  return audit.inspect(path.join(run,'saves',slot+'.json'),buyer);""")
for before, after in [
 ("assert.deepEqual(await archive(page,url,run,slot+'-after'),before,", "audit.compare(await archive(page,url,run,slot+'-after'),before,"),
 ("assert.deepEqual(await archive(page,url,run,'s08-preparation-after'),prepWorld,", "audit.compare(await archive(page,url,run,'s08-preparation-after'),prepWorld,"),
 ("assert.deepEqual(await archive(page,url,run,'s08-preview-after'),beforeReview,", "audit.compare(await archive(page,url,run,'s08-preview-after'),beforeReview,"),
 ("assert.deepEqual(await archive(page,url,run,'s08-cancel-review'),after,", "audit.compare(await archive(page,url,run,'s08-cancel-review'),after,"),
 ("assert.deepEqual(await archive(page,url,run,'s08-maintenance-review'),maintenanceBefore,", "audit.compare(await archive(page,url,run,'s08-maintenance-review'),maintenanceBefore,"),
 ("assert.deepEqual(await archive(page,url,run,'s08-continue'),finalWorld,", "audit.compare(await archive(page,url,run,'s08-continue'),finalWorld,")]:
    replace(before, after)
replace("assert(!v.advance_pending);await idle(page);return state(page,url);", "assert(!v.advance_pending);await idle(page);const s=await state(page,url);return {date:s.date,session_id:s.session_id,player:s.player,simulation_cadence:s.simulation_cadence};")
replace("""    const response=await page.request.get(url+'/'+name);assert(response.ok());const actual=await response.body();assert(actual.equals(checkout),'Wrong embedded '+name);
    result[name]={served_sha256:hash(actual),committed_sha256:hash(r.stdout),newline_normalization:'CRLF to LF for checkout-to-commit only'};""",
"""    const response=await page.request.get(url+'/'+name);
    try{assert(response.ok());const actual=await response.body();assert(actual.equals(checkout),'Wrong embedded '+name);
      result[name]={served_sha256:hash(actual),committed_sha256:hash(r.stdout),newline_normalization:'CRLF to LF for checkout-to-commit only'};
    }finally{await response.dispose();}""")
replace("'git',['show',revision", "'git',['-c','core.longpaths=true','show',revision")
replace("const response=await route.fetch();intercepted();await heldResponse;await route.fulfill({response});handled();", "const response=await route.fetch();try{intercepted();await heldResponse;await route.fulfill({response});}finally{await response.dispose();handled();}")
replace("let offer=choose(data);assert(offer,'The supplied genuine fixture must expose an affordable accessible foreign equipment lot');e.offer=copy(offer);",
        "let offer=choose(data);assert(offer,'The supplied genuine fixture must expose an affordable accessible foreign equipment lot');e.offer=copy(offer);audit.select({seller:offer.seller_nation,company:offer.company,product:offer.product});")
replace("data=await board(page,url);offer=choose(data);assert(offer,'Affordable actual stock disappeared after the stale-review day');",
        "data=await board(page,url);offer=choose(data);assert(offer,'Affordable actual stock disappeared after the stale-review day');audit.select({seller:offer.seller_nation,company:offer.company,product:offer.product});")
replace("""const after=await archive(page,url,run,'s08-purchase-after'),oldIds=new Set((before.companies.imports?.contracts||[]).map(d=>d.id));
    const added=after.companies.imports.contracts.filter(d=>!oldIds.has(d.id));assert.equal(added.length,1);const purchased=added[0];e.contract_id=purchased.id;""",
"""const after=await archive(page,url,run,'s08-purchase-after'),oldIds=new Set(before.imports.ids);
    const addedIds=after.imports.ids.filter(id=>!oldIds.has(id));assert.equal(addedIds.length,1);const purchased=contract(after,addedIds[0]);e.contract_id=purchased.id;
    audit.select({seller:offer.seller_nation,company:offer.company,product:offer.product,revision:purchased.buyer_revision});""")
replace("!nation(before,buyer).equipment?.revisions?.[purchased.buyer_revision]", "!before.buyer.revision_ids.includes(purchased.buyer_revision)")
replace("assert.deepEqual(nation(after,buyer).equipment?.learned||[],nation(before,buyer).equipment?.learned||[],", "assert.equal(after.buyer.learned_sha256,before.buyer.learned_sha256,")
replace("assert.deepEqual(nation(after,buyer).tech,nation(before,buyer).tech,", "assert.equal(after.buyer.tech_sha256,before.buyer.tech_sha256,")
replace("!nation(after,buyer).equipment?.revisions?.[purchased.buyer_revision]", "!after.buyer.revision_ids.includes(purchased.buyer_revision)")
replace("!nation(transit,buyer).equipment?.revisions?.[purchased.buyer_revision]", "!transit.buyer.revision_ids.includes(purchased.buyer_revision)")
replace("const buyerRevision=nation(finalWorld,buyer).equipment.revisions[delivered.buyer_revision];assert(buyerRevision);", "const buyerRevision=finalWorld.buyer.buyer_revision;assert(buyerRevision);assert.equal(finalWorld.requested.revision,delivered.buyer_revision);")
replace("approvedPlan=nation(maintenanceApproved,buyer).equipment.maintenance_plan", "approvedPlan=maintenanceApproved.buyer.maintenance_plan")
replace("""    const expectedApproval=copy(maintenanceBefore);nation(expectedApproval,buyer).equipment.maintenance_plan=copy(approvedPlan);
    assert.deepEqual(maintenanceApproved,expectedApproval,'Approving maintenance must only set its plan: no instant money, authority, research or property grant');""",
"""    // Compare every native field exactly except the single approved plan;
    // each parsed world is released when its audit child exits. Never format
    // a whole-world assertion diff, even if this check finds a real defect.
    audit.compareExceptMaintenance(maintenanceBefore,maintenanceApproved,'Approving maintenance must only set its plan: no instant money, authority, research or property grant');""")
replace("const priorReceiptDay=nation(maintenanceBefore,buyer).equipment.maintenance_plan?.receipt?.day??null;", "const priorReceiptDay=maintenanceBefore.buyer.maintenance_plan?.receipt?.day??null;")
replace("n=nation(observed,buyer),plan=n.equipment.maintenance_plan", "n=observed.buyer,plan=n.maintenance_plan")
replace("const evidenceDir=fs.mkdtempSync(path.join(output,'supplier-'));", """const evidenceDir=fs.mkdtempSync(path.join(output,'supplier-'));
  const telemetry=(event,detail={})=>fs.appendFileSync(path.join(evidenceDir,'progress.jsonl'),JSON.stringify({utc:new Date().toISOString(),pid:process.pid,event,stage,memory:process.memoryUsage(),...detail})+'\\n');
  const mark=value=>{stage=value;telemetry('stage');};""")
replace("let run,input,url,server,log,browser,page,stage='launch',launchError;", "let run,input,url,server,log,browser,page,stage='launch',launchError;\n  audit.configure(evidenceDir,telemetry);telemetry('runner-start',{worker:process.env.SPHERES_SUPPLIER_ARCHIVE_AUDIT||'./supplier-archive-audit.cjs'});")
text = re.sub(r"(?<![,\w])stage='([^']*)';", r"mark('\1');", text)
for name in ['preparation', 'input']:
    text = text.replace('hash(fs.readFileSync('+name+'))', 'audit.fileHash('+name+')')
replace("}catch(error){e.failed_stage=stage;", "}catch(error){telemetry('failure',{error:String(error)});e.failed_stage=stage;")
replace("}finally{if(browser)await browser.close();if(server)server.kill();if(log)log.end();}", "}finally{if(browser)await browser.close();if(server)server.kill();if(log)log.end();telemetry('runner-finished',{passed:e.passed});}")
assert 'nation(' not in text and 'worldOf(' not in text and 'expectedApproval=' not in text
target = base / 'ci-supplier-imports.cjs'
target.write_text(text, encoding='utf-8', newline='\n')
(base / 'bounded-resources.patch').write_text(''.join(difflib.unified_diff(source.splitlines(True), text.splitlines(True), fromfile='load-corrected/ci-supplier-imports.cjs', tofile='bounded/ci-supplier-imports.cjs')), encoding='utf-8')
loader = """const fs=require('node:fs'),path=require('node:path'),Module=require('node:module');
const original=ORIGINAL;
process.env.SPHERES_SUPPLIER_ARCHIVE_AUDIT=path.join(__dirname,'archive-audit.cjs');
const loaded=new Module(original,module);loaded.filename=original;loaded.paths=Module._nodeModulePaths(path.dirname(original));
loaded._compile(fs.readFileSync(path.join(__dirname,'ci-supplier-imports.cjs'),'utf8'),original);
""".replace('ORIGINAL', json.dumps(str(repo / 'tools/ui/ci-supplier-imports.cjs')))
(base / 'load-candidate.cjs').write_text(loader, encoding='utf-8', newline='\n')
for item in [target, base/'archive-audit.cjs', base/'load-candidate.cjs']:
    subprocess.run(['node', '--check', str(item)], check=True)
proof = {'executed': False, 'note': 'Syntax checked only; no browser execution. All prior failed attempts remain unchanged.',
         'original_load_corrected_sha256': hashlib.sha256(original.read_bytes()).hexdigest(),
         'files': {item.name: hashlib.sha256(item.read_bytes()).hexdigest() for item in
                   [target, base/'archive-audit.cjs', base/'archive-worker.py', base/'load-candidate.cjs', base/'bounded-resources.patch']}}
(base / 'candidate-provenance.json').write_text(json.dumps(proof, indent=2)+'\n', encoding='utf-8')
print(json.dumps(proof, indent=2))
