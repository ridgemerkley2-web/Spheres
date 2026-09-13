"""Collect completed S09 evidence and publish a source-bound completion record."""
import datetime,gzip,hashlib,importlib.util,json,os,pathlib,re,subprocess,sys
b=pathlib.Path(__file__).resolve().parent;r=b/'integration';pin,journey_file,linux_file,visual_file=sys.argv[1:5]
read=lambda p:json.loads(pathlib.Path(p).read_text(encoding='utf-8-sig'))
def sha(p):
 with pathlib.Path(p).open('rb') as f:return hashlib.file_digest(f,'sha256').hexdigest()
def key(p):return os.path.normcase(str(pathlib.Path(p).resolve()))
def node_counts(text):
 counts={}
 for label in ['tests','suites','pass','fail','cancelled','skipped','todo']:
  values=re.findall(r'^(?:#|\u2139) '+label+r' (\d+)[ \t\r]*$',text,re.MULTILINE)
  assert len(values)==1,'Missing or ambiguous Node summary: '+label
  counts[label]=int(values[0])
 assert counts['tests']>0 and counts['pass']>0 and counts['fail']==counts['cancelled']==counts['todo']==0
 assert counts['tests']==sum(counts[x] for x in ['pass','fail','cancelled','skipped','todo'])
 return counts

def git(*args):return subprocess.check_output(['git','-c','core.longpaths=true',*args],cwd=r,text=True).strip()
assert re.fullmatch('[0-9a-f]{40}',pin)
assert not git('status','--porcelain')
assert not git('diff','--name-only',pin,'HEAD','--','spheres-sim','spheres-cli','spheres-web','Cargo.toml','Cargo.lock')
native=b/'evidence/S09-final-connection-native.json';build=b/'evidence/S09-final-connection-binary.json';external=b/'evidence/S09-external-connection-1/result.json';preservation=b/'evidence/S09-preservation-closeout.json';launch=b/'S09-review-launch.json'
journey_file,linux_file,visual_file=[pathlib.Path(p).resolve() for p in [journey_file,linux_file,visual_file]]
node_file=b/'evidence/S09-final-connection-node.json';node_log=node_file.with_suffix('.log')
visual_build_file=b/'evidence/S09-final-connection-visual-binary.json'
visual_notes_file=visual_file.parent.parent/'README.md'
vendor_file=b/'evidence/S09-taskpool-vendor-review.json'
performance_file=journey_file.parent/'read-performance.json'
folder=r/'docs/campaign-certification/S09'
assert not (folder/'manifest.json').exists() and not (folder/'evidence').exists(),'S09 publication already exists'

n,build_record,ex,keep,review,journey,linux,visual=map(read,[native,build,external,preservation,launch,journey_file,linux_file,visual_file])
assert n['passed'] and n['clean_before'] and n['clean_after'] and n['preservation_passed'] and n['exit_code']==0 and n['totals']['completed_targets']>0 and '--workspace' in n['command'] and n['revision_before']==n['revision_after']==pin and n['totals']['failed']==0
assert build_record['passed'] and build_record['clean_before'] and build_record['clean_after'] and build_record['preservation_passed'] and build_record['exit_code']==0 and build_record['revision_before']==build_record['revision_after']==pin
assert ex['passed'] and ex['candidate']==pin and ex['binary_sha256']==n['binary_sha256']
assert keep['passed'] and review['runtime_revision']==pin
assert journey['passed'] and journey['build']['revision']==pin and journey['build']['binary_sha256']==build_record['binary_sha256']==review['executable_sha256']
assert linux['status']=='passed' and linux['candidate']==pin
assert visual['review_capture_completed'] and visual['expected_revision']==pin and not visual['page_errors'] and not visual['commands']
# Node qualification is a full, source-bound lane on both platforms.
windows_node=read(node_file)
assert windows_node['passed'] and windows_node['exit_code']==0 and windows_node['clean_before'] and windows_node['clean_after'] and windows_node['preservation_passed']
assert windows_node['revision_before']==windows_node['revision_after']==pin
assert windows_node['command']==['node','tools/ui/run-unit.cjs'] and sha(node_log)==windows_node['log_sha256']
windows_node_counts=node_counts(node_log.read_text(encoding='utf-8',errors='strict'))
linux_checks={c['name']:c for c in linux['checks']}
assert len(linux['checks'])==len(linux_checks)==5 and set(linux_checks)=={'native','node','master-archive','active-fixtures','party-versions'}
assert all(c['exit_code']==0 for c in linux['checks'])
assert linux_checks['native']['totals']['failed']==0 and linux_checks['native']['totals']['completed_targets']>0
assert all(linux_checks[name]['exact_one_test_passed'] for name in ['master-archive','active-fixtures','party-versions'])
assert linux['web_test_binary']['sha256']==linux['web_test_binary']['sha256_after']
linux_node=linux_checks['node'];linux_node_counts=node_counts(linux_node['summary_tail'])
linux_node_log=linux_file.parent/linux_node['log']
assert sha(linux_node_log)==linux_node['log_sha256'] and node_counts(linux_node_log.read_text(encoding='utf-8',errors='strict'))==linux_node_counts
assert linux['source_before']['head']==linux['source_after']['head']==pin and not linux['source_before']['status_porcelain'] and not linux['source_after']['status_porcelain']
assert len(ex['checks'])==3 and all(c['exit_code']==0 and c['exact_one_test_passed'] for c in ex['checks'])
assert ex['candidate_after']==pin and ex['clean_after'] and ex['native_proof_sha256']==sha(native)
# The executed driver is separately identified from the runtime source.
driver=journey['test_source'];assert driver['runtime_source_equal'] is True and re.fullmatch('[0-9a-f]{40}',driver['revision'])
journey_group=journey_file.parent.parent.name;assert journey_group.startswith('S09-browser-')
outer_file=b/'evidence'/('S09-browser-driver-'+journey_group.removeprefix('S09-browser-')+'.json');outer=read(outer_file)
assert outer['passed'] and outer['exit_code']==0 and outer['clean_after'] and outer['runtime_source_equal']
assert outer['runtime_revision']==pin and outer['driver_revision']==outer['driver_revision_after']==driver['revision']
assert outer['driver_sha256_before']==outer['driver_sha256_after']==driver['driver_sha256']
assert outer['binary_sha256_before']==outer['binary_sha256_after']==build_record['binary_sha256']
assert sha(outer_file.with_suffix('.log'))==outer['log_sha256']
assert not git('diff','--name-only',pin,driver['revision'],'--','spheres-sim','spheres-cli','spheres-web','Cargo.toml','Cargo.lock')
driver_file=r/'tools/ui/ci-research-design.cjs';assert sha(driver_file)==driver['driver_sha256']
committed_driver=subprocess.check_output(['git','-c','core.longpaths=true','show',driver['revision']+':tools/ui/ci-research-design.cjs'],cwd=r)
assert committed_driver.replace(b'\r\n',b'\n')==driver_file.read_bytes().replace(b'\r\n',b'\n'),'Executed driver is not the recorded committed test source'
assert not journey['errors'] and len(journey['commands'])==1
assert [c['kind'] for envelope in journey['commands'] for c in envelope['commands']]==['equipment_save']
assert journey['saved_draft']['id']=='draft:S09 Atlas' and review['saved_draft']==journey['saved_draft']
qualifications={key(x['path']):x['sha256'] for x in review['qualification']}
assert qualifications[key(build)]==sha(build) and qualifications[key(journey_file)]==sha(journey_file),'Review used different qualification records'
# Retain the visual observer's actual proof shape: API stamp plus the written
# independent prelaunch SHA check, not an invented hash in the result JSON.
visual_build=read(visual_build_file);visual_notes=visual_notes_file.read_text(encoding='utf-8')
assert visual_build['passed'] and visual_build['clean_before'] and visual_build['clean_after'] and visual_build['exit_code']==0
assert visual_build['revision_before']==visual_build['revision_after']==pin and visual['build']['revision']==pin[:12]
assert visual_build['binary_sha256'] in visual_notes and pin in visual_notes
assert visual['desktop_canvas_nonblank'] and visual['mobile_canvas_nonblank'] and not visual.get('http_errors')
vendor=read(vendor_file);assert vendor['passed'] and vendor['candidate']==pin and vendor['changed_upstream_files']==['src/util/task_pool.rs'] and vendor['exact_executed_regression_test'] and vendor['workspace_test_compiles_shipped_source']
# Independently verify all twelve finite read cases and their declared bars.
performance=read(performance_file);assert performance['warmups']==3 and performance['samples']==21
expected={'board','tank_standard','tank_heavy','tank_light','tank_destroyer','ground_ifv','ground_apc','ground_recon','ground_artillery','ground_air_defense','air_light_attack','air_tactical_strike'}
assert len(performance['rows'])==len(journey['read_performance'])==12 and {x['id'] for x in performance['rows']}==expected
for row in performance['rows']:
 samples=row['samples_ms'];assert len(samples)==21 and all(isinstance(x,(int,float)) and x>=0 and x<float('inf') for x in samples)
 ordered=sorted(samples);assert row['p95_ms']==ordered[(len(samples)*95+99)//100-1] and row['max_ms']==ordered[-1]
 limits=(300,750) if row['id']=='board' else (250,500)
 assert (row['p95_limit_ms'],row['max_limit_ms'])==limits and row['passed'] and row['p95_ms']<=limits[0] and row['max_ms']<=limits[1]
assert journey['read_performance']==[{k:v for k,v in row.items() if k!='samples_ms'} for row in performance['rows']]

assert sha(build_record['binary'])==build_record['binary_sha256']
assert sha(n['binary'])==n['binary_sha256']
assert review['native_verification']['exact_canonical_bytes_equal'] and review['native_verification']['ignored_paths']==[]
linked_native_proofs=[]
for field in ['browser_audit','copied_save_audit']:
 link=review['native_verification'][field];path=pathlib.Path(link['path']).resolve();assert sha(path)==link['sha256'];linked_native_proofs.append(path)
proof_inputs=[('windows_native',native),('windows_build',build),('windows_node',node_file),('windows_node_log',node_log),('windows_external',external),('linux',linux_file),('linux_node_log',linux_node_log),('browser',journey_file),('read_performance',performance_file),('visual',visual_file),('visual_build',visual_build_file),('visual_observations',visual_notes_file),('vendor_review',vendor_file),('preservation',preservation),('launch',launch),('executed_driver',driver_file)]
proof_inputs.extend(('review_native_'+str(i),path) for i,path in enumerate(linked_native_proofs,1))
proof_inputs.extend([('browser_runner',outer_file),('browser_runner_log',outer_file.with_suffix('.log'))])
required_hashes={key(path):sha(path) for _,path in proof_inputs}

sources=[]
for p in sorted((b/'evidence').glob('S09*')):
 files=[p] if p.is_file() else sorted(x for x in p.rglob('*') if x.is_file())
 for f in files:
  if f.suffix.lower() in ('.exe','.pdb'):continue
  sources.append({'source':str(f.resolve()),'name':f.relative_to(b/'evidence').as_posix()})
for p in sorted(b.iterdir()):
 if p.is_file() and ('s09' in p.name.lower()) and p.suffix.lower() in ('.py','.cjs','.json') and p.name not in ('S09-publication-selection.json',):
  sources.append({'source':str(p.resolve()),'name':'helpers/'+p.name})
review_dir=pathlib.Path(review['directory'])
for p in sorted(review_dir.rglob('*')):
 if p.is_file() and p.suffix.lower() not in ('.exe','.pdb'):
  sources.append({'source':str(p.resolve()),'name':'review/'+p.relative_to(review_dir).as_posix()})
sources.append({'source':str(driver_file.resolve()),'name':'test-source/ci-research-design.cjs'})
selected={key(x['source']) for x in sources}
assert set(required_hashes)<=selected,'Required proof absent from evidence selection: '+repr(set(required_hashes)-selected)
# Run only the collector's pure preflight before creating the selection file.
sys.dont_write_bytecode=True
collector_spec=importlib.util.spec_from_file_location('s09_publication_collector',b/'collect-s09-evidence.py')
collector=importlib.util.module_from_spec(collector_spec);collector_spec.loader.exec_module(collector)
collector.preflight(sources,folder/'evidence')
selection=b/'S09-publication-selection.json'

with selection.open('x',encoding='utf-8') as f:json.dump(sources,f,indent=2);f.write('\n')
subprocess.run([sys.executable,str(b/'collect-s09-evidence.py'),str(selection)],check=True)
folder=r/'docs/campaign-certification/S09';inventory=folder/'evidence/inventory.json';inv=read(inventory)
mapping={key(x['source']):x for x in inv['files']}
assert set(required_hashes)<=set(mapping)
assert all(mapping[path]['sha256']==digest for path,digest in required_hashes.items()),'Required proof changed during collection'
def proof(p):
 p=pathlib.Path(p).resolve();item=mapping[key(p)];assert sha(p)==item['sha256']
 return {'logical_path':'evidence/'+item['logical_path'],'sha256':item['sha256'],'bytes':item['bytes'],'storage':['evidence/'+x['file'] for x in item['storage']]}
manifest={'format':'spheres-s09-qualification','version':1,'session':'S09','status':'complete','publication_status':'collected_and_verified','prepared_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'runtime_candidate':pin,'driver_revision':journey['test_source']['revision'],'driver_sha256':journey['test_source']['driver_sha256'],'completion_commit':{'resolution':'containing_git_commit','command':'git log -1 --format=%H -- docs/campaign-certification/S09/manifest.json'},'scope':'Research explanations and legal known design suggestions, pure drafts, explicit save and exact persistence. Includes the demonstrated HTTP connection-pool startup correction. Stop before S10; not a final campaign or worldwide content certificate.','runtime_binary':{'sha256':build_record['binary_sha256'],'freshly_hashed':True},'test_binaries':{'windows':n['binary_sha256'],'linux':linux['web_test_binary']['sha256']},'native':{'windows':n['totals'],'linux':next(c['totals'] for c in linux['checks'] if c['name']=='native')},'node':{'windows':windows_node_counts,'linux':linux_node_counts},'visual_binding':{'observed_api_revision':visual['build']['revision'],'prelaunch_binary_sha256':visual_build['binary_sha256'],'basis':'Contemporaneous independent prelaunch hash observation in the visual README, matched to the clean build proof; visual result itself records the observed API stamp and settled canvases.'},'external_archives':{'windows':len(ex['checks']),'linux':len([c for c in linux['checks'] if c.get('exact_one_test_passed')]),'scope':'Original pinned master, 28 original active supplier fixtures, party-v1/equipment-v2–5 books.'},'browser_checks':journey['checks'],'commands':journey['commands'],'read_performance':journey['read_performance'],'performance_scope':'Fresh paused France; complete HTTP/JSON reads; 3 warmups then21 samples each; predeclared board p95<=300ms/max750ms and preview p95<=250ms/max500ms. No claim for a late-game or fully researched workload.','limitations':['Economical fabrication/upkeep suggestions are conditional estimates, not affordable supplier offers. All spending still needs native review.','The browser uses fresh France and one explicitly saved design; no completed research or certified revision is fabricated. Native tests supply the research-completion/frozen-revision guarantees.','Read performance is finite workload evidence; retained failures remain attributed to their own source.','Existing S08 large-save-directory scanning limitation remains; this session does not certify final campaign performance.'],'review':{'url':review['url'],'player':review['player'],'date':review['date'],'saved_draft':review['saved_draft']['id']},'proofs':{k:proof(p) for k,p in proof_inputs},'inventory':{'file':'evidence/inventory.json','sha256':sha(inventory),'files':len(inv['files']),'stored_bytes':inv['stored_bytes']}}
manifest['retained_performance_failure']={'source':'evidence/S09-browser-qualified-1/france-suFbnD/read-performance.json','board_p95_ms':313.0021999999999,'board_max_ms':324.03069999999934,'cause':'Unresolved isolated stalls; normal samples12–19ms. Observation and subsequent completed journey passed unchanged limits; no runtime performance improvement claimed.'}
with (folder/'manifest.json').open('x',encoding='utf-8') as f:json.dump(manifest,f,indent=2);f.write('\n')
print(json.dumps({'manifest':str(folder/'manifest.json'),'evidence_files':len(inv['files']),'runtime':pin}),flush=True)
