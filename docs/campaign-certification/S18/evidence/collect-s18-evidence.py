import datetime,gzip,hashlib,json,pathlib,subprocess,sys
base=pathlib.Path(__file__).resolve().parent;repo=base/'integration';dest=repo/'docs/campaign-certification/S18';pin=sys.argv[1]
def read(p):return json.loads(p.read_text(encoding='utf-8-sig'))
def sha(p):
 with p.open('rb') as f:return hashlib.file_digest(f,'sha256').hexdigest()
def git(*args):return subprocess.check_output(['git','-c','core.longpaths=true',*args],cwd=repo,text=True).strip()
testpin=git('rev-parse','HEAD')
probe='tools/ui/check_equipment_realism.cjs'
assert git('diff','--name-only',pin,testpin)==probe
assert git('show',pin+':'+probe).replace('z=strike?1.60:.85;', 'z=strike?1.60:2.55;')==git('show',testpin+':'+probe)
assert not git('status','--porcelain')
proofs={lane:read(base/f'evidence/S18-final1-{lane}.json') for lane in ['assets','art','web','fixture','binary','browser']}
for lane,p in proofs.items():
 assert p['passed'] and p['revision']==p['revision_after']==pin and p['clean_before'] and p['clean_after'],lane
 assert sha(base/f'evidence/S18-final1-{lane}.log')==p['log_sha256']
node=read(base/'evidence/S18-final2-node.json');assert node['passed'] and node['revision']==node['revision_after']==testpin and node['clean_before'] and node['clean_after']
assert sha(base/'evidence/S18-final2-node.log')==node['log_sha256'];proofs['node']=node
linux1=read(base/'evidence/S18-final1-linux/result.json');assert linux1['revision']==pin and len(linux1['checks'])==2
linux2=read(base/'evidence/S18-final2-linux/result.json');assert linux2['passed'] and linux2['revision']==testpin and len(linux2['checks'])==1
for folder,record in [('S18-final1-linux',linux1),('S18-final2-linux',linux2)]:
 for c in record['checks']:assert sha(base/'evidence'/folder/c['log'])==c['log_sha256']
web=next(c for c in linux1['checks'] if c['name']=='web');assert web['passed'] and web['source_before']==web['source_after']=={'head':pin,'status':''}
linux={'passed':True,'checks':[web,linux2['checks'][0]],'web_source':'evidence/S18-final1-linux/result.json','node_source':'evidence/S18-final2-linux/result.json','runtime_revision':pin,'test_revision':testpin,'note':'The first Node run failed only because its single-seat inlet probe used the previous mouth position. Only that test coordinate changed; full Node was rerun on both platforms. Native web, assets and browser evidence remains pinned to the identical runtime code.'}
browserfiles=list((base/'evidence/S18-browser-final1-browser').glob('staff-*/result.json'));assert len(browserfiles)==1
browserfile=browserfiles[0];browser=read(browserfile);assert browser['passed'] and browser['revision']==pin and not browser['errors']
assert len(browser['commands'])==2 and len(browser['advances'])==6 and len(browser['world_comparisons'])==10 and len(browser['envelope_comparisons'])==10
assert browser['flight_pages']['pages']==['command','aircraft','bases','reports'] and browser['flight_pages']['inspections'] and browser['flight_pages']['draft_preserved']
artfiles=list((base/'evidence/S18-art-final').glob('inspection-*/result.json'));assert len(artfiles)==1
artfile=artfiles[0];art=read(artfile);assert art['passed'] and len(art['aircraft'])==3 and not art['errors']
visual=read(base/'evidence/S18-visual-review.json');assert visual['passed'] and visual['revision']==pin
preserve=read(base/'evidence/S18-preservation-final.json');assert preserve['passed']
launch=read(base/'S18-review-launch.json');assert launch['runtime_revision']==pin
assert git('diff','--name-only','502f3d5',pin,'--','spheres-sim','Cargo.toml','Cargo.lock')==''
assert git('diff','--name-only','502f3d5',pin,'--','spheres-web/src')=='spheres-web/src/equipment_flight_view.rs'
assert not (dest/'manifest.json').exists() and not (dest/'evidence').exists()
sources=[]
def add(p):
 assert p.is_file() and not p.is_symlink()
 if p not in sources:sources.append(p)
for p in sorted((base/'evidence').glob('S18-*.json')):add(p)
for p in sorted((base/'evidence').glob('S18-*.log')):add(p)
for folder in [base/'evidence/S18-final1-linux',base/'evidence/S18-final2-linux',base/'evidence/S18-fixture-final1',browserfile.parent,artfile.parent]:
 for p in sorted(folder.rglob('*')):
  if p.is_file() and p.suffix in ['.json','.log','.png'] and p.name!='s17-staff-input.json':add(p)
for name in ['run-s18-check.py','run-s18-windows-final1.py','run-s18-windows-runtime1.py','run-s18-linux-final1.py','run-s18-linux-final2.py','launch-s18-review.py','collect-s18-evidence.py','finish-s18-docs.py','verify-s08-preservation.py','S18-review-launch.json']:add(base/name)
entries=[]
for source in sources:
 raw=source.read_bytes();compressed=len(raw)>1024*1024 and source.suffix!='.png';relative=source.relative_to(base).as_posix();stored='evidence/'+relative+('.gz' if compressed else '')
 target=dest/stored;target.parent.mkdir(parents=True,exist_ok=True);assert not target.exists();payload=gzip.compress(raw,compresslevel=9,mtime=0) if compressed else raw;target.write_bytes(payload)
 assert (gzip.decompress(payload) if compressed else payload)==raw
 entries.append({'source':relative,'bytes':len(raw),'sha256':hashlib.sha256(raw).hexdigest(),'stored':stored,'encoding':'gzip' if compressed else 'identity','stored_bytes':len(payload),'stored_sha256':sha(target)})
manifest={'format':'spheres-s18-closure/v1','session':'S18','status':'complete','candidate_revision':pin,'test_revision':testpin,'test_only_correction':{'path':probe,'before':'z=strike?1.60:.85;','after':'z=strike?1.60:2.55;','exact_source_comparison_passed':True},'completed_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'g4_earned':False,'cp1_earned':False,
 'scope':'Native campaign Command/Aircraft/Bases/Reports, exact frozen aircraft inspection, three CP1 aircraft families with meaningful 100k+ inspection geometry, cheaper LODs and validated selectable/exported assemblies. Art-only workshop retires demonstration mission state. No new simulation rule, historical coverage, performance or full-campaign certification claim.',
 'qualification':{'windows':proofs,'linux':linux,'browser':{'passed':True,'ordinary_commands':2,'ordinary_days':6,'world_comparisons':10,'envelope_comparisons':10,'errors':[],'flight_pages':browser['flight_pages'],'binary_sha256':browser['binary_sha256']},'art':art,'visual_review':visual,'preservation':preserve},
 'source_continuity':{'simulation_unchanged_since':'502f3d5fe680a5d92649d484e80ac2ac23535f64','checked_paths':['spheres-sim','Cargo.toml','Cargo.lock'],'changed_native_paths':['spheres-web/src/equipment_flight_view.rs'],'native_change':'Expose existing frozen spec in the read-only flight board and assert exact equality while retaining save-purity check. Full native web and six-day native/browser scenario rerun.','inherited_simulation_evidence':'docs/campaign-certification/S17/manifest.json','inherited_manifest_sha256':sha(repo/'docs/campaign-certification/S17/manifest.json')},
 'review':launch,'prior_attempts':'Development logs retain failures caused by intentionally superseded aircraft pins/assets and controls moving to dedicated flight pages. Art review also fixed occluded inlets and camera framing. Only final source-pinned passing records qualify this session.',
 'retention':{'files':len(entries),'stored_bytes':sum(e['stored_bytes'] for e in entries),'excluded':'Compiled binaries and duplicate canonical worlds. Exact raw native/browser saves and audit metadata are retained; large text archives use reconstructable gzip.','files_inventory':entries}}
(dest/'manifest.json').write_text(json.dumps(manifest,indent=2)+'\n',encoding='utf8');(dest/'.gitattributes').write_text('evidence/** -text -whitespace\n',encoding='utf8');print(json.dumps({'files':len(entries),'stored_bytes':manifest['retention']['stored_bytes']}))
