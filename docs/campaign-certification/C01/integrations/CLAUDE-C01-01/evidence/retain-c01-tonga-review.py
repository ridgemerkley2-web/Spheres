import datetime,hashlib,json,pathlib,subprocess
base=pathlib.Path(__file__).resolve().parent;repo=base/'integration'
source=base/'evidence/C01-tonga-review-20260921'
dest=repo/'docs/campaign-certification/C01/integrations/CLAUDE-C01-01'
def read(p):return json.loads(p.read_text(encoding='utf8'))
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def git(*args):return subprocess.check_output(['git','-c','core.longpaths=true',*args],cwd=repo,text=True).strip()
proof=read(source/'qualification-final/result.json');browser=read(source/'qualification-final/browser/result.json')
assert proof['passed'] and browser['passed'] and proof['source_revision']==browser['revision']==git('rev-parse','HEAD')
assert proof['clean_before'] and proof['clean_after'] and browser['clean_before'] and browser['clean_after']
assert len(browser['countries'])==9 and not browser['errors'] and browser['society_observations']['terms_not_inferred']
assert len(browser['authored_error_cases'])==2
for c in proof['checks']:assert c['passed'] and sha(source/'qualification-final'/c['log'])==c['sha256']
for file,digest in browser['source_files'].items():assert sha(repo/file)==digest,file
assert not git('diff','939e8f9','--','spheres-sim','spheres-web','Cargo.toml','Cargo.lock','docs/planning/campaign-pathway.json')
originals=read(source/'source-fetch.json')
assert originals['sources'][1]['sha256']=='ff8af75772e2f6dd0f3eaaa0b300239bc2925a867dff8f9926142299dee90973'
review={'reviewer':'Codex','reviewed_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),
 'sources':originals['sources'],
 'visual_pages_one_based':{'trial':[1,2,4,33,34],'appeal':[9,12,15]},
 'html_sources_checked':['https://data.ipu.org/parliament/TO/TO-LC01/election/TO-LC01-E20171116/','https://data.ipu.org/parliament/TO/TO-LC01/election/TO-LC01-E20211118/'],
 'html_method':'Web tool text retrieval and relevant election narratives read; no new original-response hash claimed.',
 'screenshots_visually_read':['tonga-society-1440.png','tonga-society-390.png','tonga-society-320.png'],
 'decision':'Accept the bounded dated observations and provisional grouping; retain all unresolved legal identities, full leadership terms and appointment dates.',
 'scope':'Research and atlas only; no country, character-coverage or campaign certificate completion.'}
assert not (dest/'manifest.json').exists()
paths=[source/'source-fetch.json']
for folder in [source/'qualification',source/'qualification-final']:
    paths.extend(p for p in sorted(folder.rglob('*')) if p.is_file() and p.suffix in ['.json','.log','.png'])
paths.extend(base/name for name in ['review-c01-tonga-sources.py','qualify-c01-tonga.py','qualify-c01-tonga-final.py','retain-c01-tonga-review.py'])
inventory=[]
for p in paths:
    relative=p.relative_to(base).as_posix();target=dest/'evidence'/relative
    target.parent.mkdir(parents=True,exist_ok=True);assert not target.exists();target.write_bytes(p.read_bytes())
    inventory.append({'file':target.relative_to(dest).as_posix(),'bytes':target.stat().st_size,'sha256':sha(target)})
manifest={'format':'spheres-research-integration/v1','packet':'CLAUDE-C01-01','status':'accepted_and_integrated','submitted_commit':originals['reviewed_commit'],
 'merge_commit':git('rev-parse','e759e69'),'qualified_source':proof['source_revision'],'review':review,'validation':proof,
 'counts':{'tonga_tests':18,'campaign_research_tests':16,'atlas_node_tests':11,'browser_countries':9,'widths':[1440,390,320]},
 'runtime_unchanged_since':'939e8f9b3467ffc1666fea19fd4670257ea1e612','parent_c01_complete':False,'cp1_earned':False,
 'source_and_test_files':browser['source_files'],'retention':inventory,
 'excluded':'Source PDF bodies and source-page renders remain local. No source artwork is republished. The evidence includes research UI captures, checksums, logs and scripts only.'}
(dest/'manifest.json').write_text(json.dumps(manifest,indent=2)+'\n',encoding='utf8')
(dest/'.gitattributes').write_text('evidence/** -text -whitespace\n',encoding='utf8')
for e in inventory:assert sha(dest/e['file'])==e['sha256']
print(json.dumps({'accepted':manifest['packet'],'retained_files':len(inventory),'bytes':sum(e['bytes'] for e in inventory)}))
