"""Retain exact S17 proofs, logs and reconstructable campaign checkpoints.
Run only after final qualification, visual review and preserved-original checks.
Compiled binaries and duplicate canonical worlds are identified, not copied.
"""
import datetime, gzip, hashlib, json, pathlib, shutil, subprocess, sys
base=pathlib.Path(__file__).resolve().parent; repo=base/'integration'
dest=repo/'docs/campaign-certification/S17'
pin=sys.argv[1]
logic_pin='8b31913e7db3dab499ae7a3dce1a753ae8654bdb'
node_pin='1b7007a8b7b4ca4b960b403b76868cd134dabdd6'
def read(p): return json.loads(p.read_text(encoding='utf-8-sig'))
def sha_bytes(b): return hashlib.sha256(b).hexdigest()
def sha(p):
    with p.open('rb') as f: return hashlib.file_digest(f,'sha256').hexdigest()
proofs={lane:read(base/f'evidence/S17-final{4 if lane in ["sim","integration"] else 5 if lane=="node" else 6}-{lane}.json') for lane in ['fixture','binary','web','sim','integration','node','browser']}
for lane,p in proofs.items():
    expected=logic_pin if lane in ['sim','integration'] else node_pin if lane=='node' else pin
    assert p['passed'] and p['revision']==expected and p['clean_before'] and p['clean_after'] and p['revision_after']==expected
linux_logic=read(base/'evidence/S17-final4-linux/result.json')
linux_node=read(base/'evidence/S17-final5-linux/result.json')
linux=read(base/'evidence/S17-final6-linux/result.json')
assert linux_logic['passed'] and linux_logic['revision']==logic_pin and len(linux_logic['checks'])==4
assert linux_node['passed'] and linux_node['revision']==node_pin and len(linux_node['checks'])==2
assert linux['passed'] and linux['revision']==pin and len(linux['checks'])==1
changed=subprocess.check_output(['git','diff','--name-only',logic_pin,pin],cwd=repo,text=True).splitlines()
assert sorted(changed)==['spheres-web/src/equipment_flight_view.rs','spheres-web/ui/equipment-ui.css','tools/ui/ci-military-staff.cjs'],changed
node_changed=subprocess.check_output(['git','diff','--name-only',node_pin,pin],cwd=repo,text=True).splitlines()
assert node_changed==['spheres-web/src/equipment_flight_view.rs'],node_changed
view='spheres-web/src/equipment_flight_view.rs'
old=subprocess.check_output(['git','show',node_pin+':'+view],cwd=repo)
new=subprocess.check_output(['git','show',pin+':'+view],cwd=repo)
assert old.replace(b'service_money(p.committed_bn)',b'company_money(p.committed_bn)').replace(b'service_money(p.purchase_limit_bn)',b'company_money(p.purchase_limit_bn)')==new
bridge={'simulation_revision':logic_pin,'node_revision':node_pin,'interface_revision':pin,'changed_paths':changed,'node_changed_paths':node_changed,'scope':'Only report CSS, screenshot positioning and two pure currency-format calls differ from the simulation candidate. Simulation/integration retain final4 qualification; unchanged JavaScript/CSS retain final5 Node qualification. Final native web, fixture, binary and browser checks qualify the displayed build.'}
matches=list((base/'evidence/S17-browser-final6-browser').glob('staff-*/result.json')); assert len(matches)==1
browserfile=matches[0]; browser=read(browserfile); browserdir=browserfile.parent
assert browser['passed'] and browser['revision']==pin and len(browser['commands'])==2 and len(browser['advances'])==6
assert browser['errors']==[] and len(browser['world_comparisons'])>=9 and len(browser['envelope_comparisons'])>=9
visual=read(base/'evidence/S17-visual-review.json'); assert visual['passed'] and visual['revision']==pin
preservation=read(base/'evidence/S17-preservation-final.json'); assert preservation['passed']
launch=read(base/'S17-review-launch.json'); assert launch['runtime_revision']==pin
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=repo,text=True).strip()==pin
assert not (dest/'manifest.json').exists() and not (dest/'evidence').exists()
sources=[]
def add(p):
    assert p.is_file() and not p.is_symlink()
    if p not in sources: sources.append(p)
for p in sorted((base/'evidence').glob('S17-*.json')): add(p)
for p in sorted((base/'evidence').glob('S17-*.log')): add(p)
for directory in [base/'evidence/S17-final6-linux',base/'evidence/S17-final5-linux',base/'evidence/S17-final4-linux',base/'evidence/S17-final3-linux',base/'evidence/S17-final1-linux',base/'evidence/S17-fixture-final6',browserdir]:
    for p in sorted(directory.rglob('*')):
        if p.is_file() and p.suffix in ['.json','.log','.png'] and p.name!='s17-staff-input.json': add(p)
for name in ['run-s17-check.py','run-s17-windows-final4.py','run-s17-linux-final4.py','run-s17-windows-final5.py','run-s17-linux-final5.py','run-s17-windows-final6.py','run-s17-linux-final6.py','launch-s17-review.py','collect-s17-evidence.py','finish-s17-docs.py','verify-s08-preservation.py','S17-review-launch.json']:
    add(base/name)
for p in sorted(base.glob('S17-*-preflight*.log')): add(p)
for p in sources:
    if p.suffix=='.json' and p.name.startswith(('S17-final4-','S17-final5-','S17-final6-')) and p.parent==base/'evidence':
        proof=read(p); assert sha(p.with_suffix('.log'))==proof['log_sha256']
for c in linux['checks']: assert sha(base/'evidence/S17-final6-linux'/c['log'])==c['log_sha256']
for c in linux_node['checks']: assert sha(base/'evidence/S17-final5-linux'/c['log'])==c['log_sha256']
for c in linux_logic['checks']: assert sha(base/'evidence/S17-final4-linux'/c['log'])==c['log_sha256']
entries=[]
for source in sources:
    relative=source.relative_to(base).as_posix(); raw=source.read_bytes()
    stored='evidence/'+relative
    compressed=len(raw)>1024*1024 and source.suffix!='.png'
    if compressed: stored+='.gz'
    target=dest/stored; target.parent.mkdir(parents=True,exist_ok=True)
    assert not target.exists()
    payload=gzip.compress(raw,compresslevel=9,mtime=0) if compressed else raw
    target.write_bytes(payload)
    assert (gzip.decompress(target.read_bytes()) if compressed else target.read_bytes())==raw
    entries.append({'source':relative,'bytes':len(raw),'sha256':sha_bytes(raw),'stored':stored,'encoding':'gzip' if compressed else 'identity','stored_bytes':len(payload),'stored_sha256':sha_bytes(payload)})
fixture=read(base/'evidence/S17-fixture-final6/manifest.json')
italian=[o for o in fixture['missions']['orders'][fixture['initial_orders']:] if o['nation']=='Italy' and (o.get('report') or {}).get('stores_used',0)>0]
assert italian
manifest={'format':'spheres-s17-closure/v1','status':'complete','session':'S17','candidate_revision':pin,
    'completed_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'g4_earned':False,'cp1_earned':False,'s18_started':False,
    'scope':'Ordinary-rule autonomous procurement, company development, support, basing and supported air operations. Isolated major/small and domestic/imported acquisition tests plus a six-day authored S16 France/Italy continuation, exact native/browser save comparisons, reviewed enable/pause and desktop/narrow visual checks. No unassisted 1990-2035, historical-content, performance, human-usability or release-certification claim.',
    'qualification':{'windows':proofs,'linux':linux,'linux_node':linux_node,'linux_simulation':linux_logic,'source_continuity':bridge,'browser':{'passed':True,'ordinary_commands':2,'ordinary_days':6,'world_comparisons':len(browser['world_comparisons']),'envelope_comparisons':len(browser['envelope_comparisons']),'errors':browser['errors'],'binary_sha256':browser['binary_sha256']},'visual_review':visual,'preservation':preservation},
    'autonomous_italian_missions':italian,'italian_staff_report':fixture['staff']['plans']['Italy'],
    'review':{'url':launch['url'],'runtime_revision':pin,'executable_sha256':launch['executable_sha256'],'save_copy':launch['save_copy'],'native_verification':launch['native_verification']},
    'inherited_s16':{'path':'docs/campaign-certification/S16/manifest.json','sha256':sha(repo/'docs/campaign-certification/S16/manifest.json')},
    'prior_attempts':'Final1 and final2 fixtures failed on unfunded Italian servicing; final1 Linux was intentionally cancelled before completion and is nonqualifying even if its partial record says running. Third candidate fixes ordinary within-Defense funding and initial review timing but its new budget test lacked proper enrollment and its browser request recorder failed. Final4 passes automated qualification but visual inspection identified oversized country-report text and misplaced narrow screenshots. Final5 corrects those; final6 fixes only two currency-format calls so review totals are not labeled per day. Exact unchanged simulation/integration and Node proofs are inherited as recorded. Earlier logs are retained as failed or superseded context only.',
    'retention':{'files':len(entries),'source_bytes':sum(x['bytes'] for x in entries),'stored_bytes':sum(x['stored_bytes'] for x in entries),'excluded':'Compiled binaries and duplicate canonical world files; binary hashes/source pins and exact raw campaign archives plus audit metadata are retained. Original failed fixture raw archives remain outside the repository; their failure logs/proofs are retained.','files_inventory':entries}}
(dest/'manifest.json').write_text(json.dumps(manifest,indent=2)+'\n',encoding='utf8')
(dest/'.gitattributes').write_text('evidence/** -text -whitespace\n',encoding='utf8')
print(json.dumps({'manifest':str(dest/'manifest.json'),'files':len(entries),'stored_bytes':manifest['retention']['stored_bytes']},indent=2))

