"""Publish selected S10.b proofs only after all declared qualifications pass."""
import datetime, hashlib, importlib.util, json, pathlib, subprocess
base=pathlib.Path(__file__).resolve().parent;repo=base/'integration'
dest=repo/'docs/campaign-certification/S10/b'
runtime='7aaa4517fbcc1c256481c2eefc5ef5e1c4153303'
git=lambda *a:subprocess.check_output(['git','-c','core.longpaths=true',*a],cwd=repo,text=True).strip()
load=lambda p:json.loads(pathlib.Path(p).read_text(encoding='utf-8'))
sha=lambda p:hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest()
assert not dest.exists() and not git('status','--porcelain')
head=git('rev-parse','HEAD')
assert not git('diff','--name-only',runtime,head,'--','spheres-sim','spheres-cli','spheres-web','Cargo.toml','Cargo.lock')
matrixroots=list((base/'evidence/S10b-matrix-final-3').glob('matrix-*'));assert len(matrixroots)==1
matrixroot=matrixroots[0];matrix=load(matrixroot/'result.json')
linuxroot=base/'evidence/S10b-linux-final-1';linux=load(linuxroot/'runner-result.json')
intakeroot=base/'evidence/S10b-intake-final-1';intake=load(intakeroot/'result.json')
assert matrix['passed'] and matrix['full_eight_country_matrix'] and len(matrix['cases'])==8
assert linux['status']=='passed' and len(linux['checks'])==9 and intake['passed']
assert not git('diff','--name-only',linux['candidate'],head,'--','tools/avatars','docs/campaign-certification/C01')
visual=load(base/'evidence/S10b-visual-review.json');assert visual['reviewed']
selection=[]
def add(path,name):
    path=pathlib.Path(path);assert path.is_file()
    selection.append({'source':str(path),'name':name})
for folder,label in [(linuxroot,'linux'),(intakeroot,'intake')]:
    for file in sorted(folder.iterdir()):
        if file.is_file():add(file,label+'/'+file.name)
add(matrixroot/'result.json','browser/result.json')
for row in matrix['cases']:
    case=pathlib.Path(row['result']).parent;detail=load(case/'result.json');assert row['passed'] and detail['passed']
    assert row['result_sha256']==sha(case/'result.json')
    label='browser/'+row['nation']
    for file in sorted(case.iterdir()):
        if file.is_file() and file.suffix in {'.json','.jsonl','.png','.log'}:add(file,label+'/'+file.name)
    for folder in ['archive-audit','server/audit-captures']:
        for file in sorted((case/folder).rglob('*')):
            if file.is_file():add(file,label+'/'+folder+'/'+file.relative_to(case/folder).as_posix())
    add(case/'server/saves'/(detail['saved_slot']+'.json'),label+'/named-campaign.json')
for attempt in [1,2]:
    firstroots=list((base/f'evidence/S10b-matrix-final-{attempt}').glob('matrix-*'));assert len(firstroots)==1
    first=firstroots[0];prefix=f'development-attempt-{attempt}';add(first/'result.json',prefix+'/result.json')
    for case in sorted(first.iterdir()):
        if not case.is_dir() or not (case/'result.json').exists():continue
        data=load(case/'result.json');label=prefix+'/'+data['nation']
        add(case/'result.json',label+'/result.json')
        for file in sorted(case.glob('*.png')):add(file,label+'/'+file.name)
        if not data['passed']:
            for name in ['failure.html','progress.jsonl']:
                if (case/name).exists():add(case/name,label+'/'+name)
            for file in sorted((case/'server/audit-captures').rglob('*.json')):add(file,label+'/audit-captures/'+file.name)
for label in ['final-1','final-2','final-3']:
    for suffix in ['json','log']:add(base/f'evidence/S10b-matrix-driver-{label}.{suffix}',f'checks/matrix-driver-{label}.{suffix}')
for name in ['S10b-visual-review.json','S10b-matrix-visual-review-1.json','S10b-driver-diagnosis.json','S10b-driver-diagnosis-2.json','S10b-preservation-final.json']:
    add(base/'evidence'/name,'checks/'+name)
for name in ['run-s10b-linux.py','run-s10b-matrix.py','run-s10b-intake-checks.py','publish-s10b.py','verify-s10b-publication.py','collect-s09-evidence.py','verify-s08-preservation.py']:
    add(base/name,'runners/'+name)
selectionpath=base/'evidence/S10b-publication-selection-final.json'
assert not selectionpath.exists();selectionpath.write_text(json.dumps(selection,indent=2)+'\n',encoding='utf-8')
spec=importlib.util.spec_from_file_location('collector',base/'collect-s09-evidence.py');collector=importlib.util.module_from_spec(spec);spec.loader.exec_module(collector)
dest.mkdir(parents=True,exist_ok=False)
collector.collect(selectionpath,dest/'evidence')
inventorypath=dest/'evidence/inventory.json';inventory=load(inventorypath)
inventory['scope']='S10.b Linux workspace, Node and archive checks; source-backed intake validation; final eight-country ordinary startup browser matrix, selected original driver failures, native snapshots and visual review. No long-run campaign or C01/G2 completion is claimed.'
inventorypath.write_text(json.dumps(inventory,indent=2)+'\n',encoding='utf-8')
(dest/'.gitattributes').write_text('# Preserve original evidence bytes, including original whitespace.\nevidence/** -text -whitespace\n',encoding='utf-8')
research=load(repo/'docs/campaign-certification/C01/research-index.json')
manifest={'format':'spheres-s10b-increment/v1','status':'increment_complete_parent_open',
    'completed_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'runtime_revision':runtime,
    'research_and_linux_revision':linux['candidate'],'browser_driver_revision':matrix['test_source']['revision'],
    'publication_parent_revision':head,'complete_runtime_source_equal':True,'runtime_or_roster_changed':False,
    'c01_complete':False,'s10_complete':False,'g2_earned':False,'s11_started':False,
    'research_counts':research['counts'],'research_index_sha256':sha(repo/'docs/campaign-certification/C01/research-index.json'),
    'qualification':{'linux_native':linux['checks'][0]['totals'],'linux_checks_passed':len(linux['checks']),
        'intake_tests':16,'browser_country_cases':8,'browser_elapsed_days':0,
        'browser_binary_sha256':matrix['binary_sha256'],'windows_runtime_evidence':'../manifest.json',
        'preservation':load(base/'evidence/S10b-preservation-final.json')['passed']},
    'evidence':{'inventory':'evidence/inventory.json','inventory_sha256':sha(inventorypath),
        'logical_files':len(inventory['files']),'stored_bytes':inventory['stored_bytes']},
    'remaining':['Complete C01 all-organization censuses, dated leadership histories and jurisdiction reconciliation; the new observations are partial.',
        'Reconcile sourced country leadership and succession coverage with S10 acceptance before claiming G2.',
        'Improve narrow review table presentation: After values require the internal horizontal scroller.']}
(dest/'manifest.json').write_text(json.dumps(manifest,indent=2)+'\n',encoding='utf-8')
rows=[]
for case in matrix['cases']:
    detail=load(case['result']);quote=detail['confirmed_review']
    rows.append(f"| {detail['name']} | {quote['title']} | {quote['price_pc']:.4f} | {quote['money_cost_bn']*1000:.3f} |")
native=linux['checks'][0]['totals']
readme=f'''# S10.b - Source-backed discovery and country government checks

**S10.b complete; S10, C01 and G2 remain open.** This increment adds research
intake code, sourced discovery packets and qualification coverage. It does not
change the simulation, playable party roster, portraits or saved campaign data.
S11 has not started.

## Research delivered

The [C01 discovery intake](../../C01/research/README.md) contains 584 organization
observations and 11 institution observations for France, Tonga and Saudi Arabia.
Its 615 claims cite 26 primary sources. The generated index assigns every exact
identity once to 61 batches of at most ten, while leaving unrepresented totals
unknown. These are research observations, not 595 verified political parties or
complete leader histories.

The France importer pins the original official CNCCFP release and checks its
checksum. The validator checks source/claim ownership, country and game-row
references, historical dates, role separation and unresolved coverage. It cannot
establish source truth or exhaustiveness. The cutoff stays 7 September 2026;
later research access does not extend a historical term or fictional eligibility.

## Government journeys verified

All eight independent fresh campaigns passed on the unchanged S10 runtime. Each
used ordinary menus to inspect its own government and a foreign government,
browse the year-2000 leadership reference, review and cancel a legal decision,
confirm it once, then use named Save, Load and Continue.

Whole native-world comparisons include dates, saved incumbents, finances, RNG
and all other serialized fields. Viewing/cancelling remained pure; confirmed
costs and every quoted effect matched the native outcome. Each confirmed action
produced a dated result. The matrix advanced **zero campaign days** and granted
no resources. It is startup coverage, not a 1990-2035 campaign qualification or
a new Russia-activation test.

| Country | Confirmed native decision | PC spent | Public payment, $ million |
| --- | --- | ---: | ---: |
{chr(10).join(rows)}

Each case includes desktop and 390px screenshots. The first driver attempt's
four electoral cases passed; four institutional cases completed their commands
but failed a checker assumption about the electoral standing-description text.
The corrected checker reads institutional standing from the saved pillar values
and retains every effect assertion. A second attempt exposed its own JSON-copy
conversion of negative zero to zero when comparing the resumed board; direct
Playwright value transport now preserves that value. Native whole-save equality
already passed in that attempt. Both original failures are preserved alongside
the successful rerun; no runtime fix or relaxed threshold was needed.

## Qualification and provenance

- Linux: {native['passed']} native tests passed, {native['failed']} failed,
  {native['ignored']} explicitly ignored, across {native['completed_targets']}
  completed targets; the full Node suite and three external archive checks passed.
- All 16 census/intake tests passed on Windows and Linux. Original census,
  pinned France import and research index reproduce exactly on both platforms.
- Windows runtime tests are the existing [S10.a proof](../manifest.json).
  The entire runtime source tree remains byte-equivalent after Git newline
  normalization; the served browser assets also matched checkout bytes exactly.
- Protected original saves, fixture binaries and source worktrees retained their
  recorded hashes. Matrix servers were disposable and stopped after their cases.

Runtime: `{runtime}`. Research and Linux source: `{linux['candidate']}`.
Final browser driver: `{matrix['test_source']['revision']}`.
The [manifest](manifest.json) and [evidence inventory](evidence/inventory.json)
bind the precise logs, native snapshots, screenshots and source revisions.

## Remaining work

Complete the country organization censuses, reconcile jurisdiction and game
identities, and source distinct party/legislative/executive histories before new
characters or succession rules are accepted. The eight startup checks do not
establish complete historical leadership or succession coverage. Longer campaign qualification remains part of the broader pathway.

Visual follow-up remains: narrow review tables require horizontal scrolling to
see After values, some captures show sticky tabs/toasts overlapping scrolled
content, and Tonga still has a portrait placeholder. These findings are recorded,
not counted as finished artwork or polished narrow-layout acceptance.
'''
(dest/'README.md').write_text(readme,encoding='utf-8',newline='\n')
pathwaypath=repo/'docs/planning/campaign-pathway.json';pathway=load(pathwaypath)
session=next(s for s in pathway['sessions'] if s['id']=='S10')
increment=next(s for s in session['increments'] if s['id']=='S10.b');increment.update(status='complete',evidence='docs/campaign-certification/S10/b/manifest.json')
session['remaining']=manifest['remaining'][:2]
pathway['execution'].update(status='stopped',stop_boundary='S10.b',next_session_requires_instruction=True)
assert session['status']=='in_progress' and next(s for s in pathway['sessions'] if s['id']=='C01')['status']=='in_progress'
assert pathway['last_completed_session']=='S09' and pathway['next_session']=='S10'
pathwaypath.write_text(json.dumps(pathway,indent=2,ensure_ascii=False)+'\n',encoding='utf-8',newline='\n')
print(json.dumps(manifest,indent=2))
