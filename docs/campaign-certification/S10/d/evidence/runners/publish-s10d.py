"""Publish a new, bounded S10.d checkpoint after its declared checks pass."""
import datetime,gzip,hashlib,importlib.util,json,pathlib,subprocess,sys
BASE=pathlib.Path(__file__).resolve().parent
REPO=BASE/'integration'
DEST=REPO/'docs/campaign-certification/S10/d'
PIN='6ab4d2fccd1128cff14ecb7470aac82b9105e291'
LOGIC='2b42cbeb44774ecdd31ca049cff3e89647269c73'
def read(p):return json.loads(pathlib.Path(p).read_text(encoding='utf-8-sig'))
def sha(p):
    with pathlib.Path(p).open('rb') as f:return hashlib.file_digest(f,'sha256').hexdigest()
def git(*args):return subprocess.check_output(['git','-c','core.longpaths=true',*args],cwd=REPO,text=True).strip()
def module(name,file):
    spec=importlib.util.spec_from_file_location(name,BASE/file);m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m);return m
def passed(p):
    r=read(p);assert r.get('passed') is True,str(p);return r
def proof(name,pin):
    p=BASE/'evidence'/name;r=passed(p)
    assert r['revision']==r['revision_after']==pin and r['clean_before'] and r['clean_after']
    assert sha(p.with_suffix('.log'))==r['log_sha256'];return r
def audit_case(case,fixture=None):
    roots=[case.resolve()]+([fixture.resolve()] if fixture else [])
    records=list((case/'archive-audit').glob('*.json'))
    assert records
    for p in records:
        r=read(p);assert r['canonical']['ignored_paths']==[]
        for key in ['input','canonical']:
            part=r[key];source=pathlib.Path(part['path']).resolve()
            assert any(source.is_relative_to(root) for root in roots)
            assert sha(source)==part['sha256'] and source.stat().st_size==part['bytes']
    comparisons=[json.loads(line) for line in (case/'progress.jsonl').read_text(encoding='utf8').splitlines() if json.loads(line).get('event')=='exact-native-comparison']
    for r in comparisons:
        assert r['equal'] and r['left']['ignored_paths']==r['right']['ignored_paths']==[]
        assert r['left']['sha256']==r['right']['sha256']
        assert pathlib.Path(r['left']['path']).read_bytes()==pathlib.Path(r['right']['path']).read_bytes()
    return {'inspections':len(records),'exact_comparisons':len(comparisons),'ignored_paths':[]}
def main():
    assert git('rev-parse','HEAD')==PIN and not git('status','--porcelain')
    assert not DEST.exists()
    delta=git('diff','--name-only',LOGIC,PIN).splitlines()
    assert delta==['spheres-web/ui/agency.css','tools/ui/ci-diplomatic-inbox.cjs']
    web=proof('S10-d-web-2.json',PIN);node=proof('S10-d-node.json',LOGIC)
    agency=proof('S10-d-agency.json',LOGIC);binary=proof('S10-d-binary-2.json',PIN)
    assert web['totals']=={'passed':371,'failed':0,'ignored':12,'completed_targets':2}
    assert node['totals']=={'tests':1538,'pass':1537,'fail':0,'skipped':1}
    assert agency['totals']['passed']==12 and agency['totals']['failed']==0
    linux=passed(BASE/'evidence/S10d-linux-final/result.json')
    assert linux['revision']==LOGIC and len(linux['checks'])==2
    for c in linux['checks']:
        assert c['exit_code']==0 and not c['totals']['failed']
        assert sha(BASE/'evidence/S10d-linux-final'/c['log'])==c['log_sha256']
    content=passed(BASE/'evidence/S10d-content-final/result.json')
    assert content['revision']==LOGIC and content['clean_after'] and len(content['checks'])==9
    for c in content['checks']:assert c['exit_code']==0 and sha(BASE/'evidence/S10d-content-final'/c['log'])==c['log_sha256']
    inbox_wrapper=passed(BASE/'evidence/S10d-inbox-final-2/result.json')
    ordinary_wrapper=proof('S10-d-browser-2.json',PIN)
    assert inbox_wrapper['revision']==PIN and inbox_wrapper['clean_after']
    cases={}
    for label,root in [('inbox',BASE/'evidence/S10d-inbox-final-2/browser'),('ordinary',BASE/'evidence/S10-browser-d-browser-2')]:
        results=list(root.glob('*/result.json'));assert len(results)==1
        case=results[0].parent;r=passed(results[0])
        assert r['build']['revision']==PIN and r['build']['binary_sha256']==binary['binary']['sha256']
        assert r['test_source']['revision']==PIN and not r['errors']
        cases[label]=(case,r,audit_case(case,BASE/'evidence/S10d-inbox-fixture-2' if label=='inbox' else None))
    inbox=cases['inbox'][1]
    assert len(inbox['cancelled_reviews'])==6 and len(inbox['commands'])==1 and inbox['advances']==[]
    assert inbox['settled_notice']['focused'] and inbox['settled_notice']['visible']
    launch=read(BASE/'S10d-review-launch.json')
    assert launch['runtime_revision']==PIN and launch['executable_sha256']==binary['binary']['sha256']
    assert launch['native_verification']['exact_bytes_match_final_browser']
    preserve=passed(BASE/'evidence/S10d-preservation-final.json')
    visual=passed(BASE/'evidence/S10d-visual-review.json')
    items=[]
    def add(p,name):items.append({'source':str(p),'name':name})
    def tree(p,name):
        for file in sorted(p.rglob('*')):
            if file.is_file():
                assert file.suffix.lower() not in {'.exe','.pdb','.dll'}
                add(file,name+'/'+file.relative_to(p).as_posix())
    for stem in ['S10-d-web','S10-d-web-2','S10-d-node','S10-d-agency','S10-d-binary-2','S10-d-browser-2']:
        for ext in ['.json','.log']:add(BASE/'evidence'/(stem+ext),'windows/'+stem+ext)
    for source,label in [('S10d-linux-final','linux'),('S10d-content-final','content'),('S10d-inbox-fixture-2','authored-native-fixture'),('S10d-inbox-final-2','authored-browser'),('S10-browser-d-browser-2','ordinary-browser')]:
        tree(BASE/'evidence'/source,label)
    for name in ['S10d-preservation-before.json','S10d-preservation-final.json','S10d-visual-review.json','S10d-final-targeted-ui.log']:
        add(BASE/'evidence'/name,'review/'+name)
    add(BASE/'S10d-review-launch.json','review/launch.json')
    tree(pathlib.Path(launch['directory'])/'native-verification','review/native-verification')
    for name in ['run-s10-check.py','run-s10d-linux.py','run-s10d-content.py','run-s10d-inbox-2.py','launch-s10d-review-2.py','collect-s09-evidence.py','publish-s10d.py','verify-s10d-publication.py']:
        add(BASE/name,'runners/'+name)
    add(BASE/'evidence/S10d-inbox-final/browser/authored-france-osIwBQ/reply-desk-320.png','development/before-close-fix-320.png')
    DEST.mkdir()
    (DEST/'.gitattributes').write_text('evidence/** -text -whitespace\n',encoding='utf8')
    selection=BASE/'evidence/S10d-selection.json';selection.write_text(json.dumps(items,indent=2)+'\n',encoding='utf8')
    module('collector','collect-s09-evidence.py').collect(selection,DEST/'evidence')
    inventory=read(DEST/'evidence/inventory.json')
    inventory['scope']='Complete selected S10.d native/interface/content logs, final authored and ordinary browser archives and exact comparisons. Earlier complete native/Node checks use the same Rust/JavaScript logic before the sole final CSS rule. Compiled binaries are identified by SHA and not bundled.'
    (DEST/'evidence/inventory.json').write_text(json.dumps(inventory,indent=2)+'\n',encoding='utf8')
    manifest={'format':'spheres-s10d-increment/v1','status':'increment_complete_parent_open',
        'created_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'runtime_revision':PIN,
        'logic_revision':LOGIC,'final_delta':delta,'binary_sha256':binary['binary']['sha256'],
        'c01_complete':False,'s10_complete':False,'g2_earned':False,'s11_started':False,
        'historical_role_or_gameplay_authorization_changed':False,'browser_elapsed_days':0,
        'qualification':{'windows_web':web['totals'],'windows_node':node['totals'],'windows_agency':agency['totals'],
            'linux_checks':[{'name':c['name'],'totals':c['totals']} for c in linux['checks']],
            'content_tests':40,'content_commands':9,'authored_browser':cases['inbox'][2],'ordinary_browser':cases['ordinary'][2],
            'authored_offers':3,'cancelled_choices':6,'confirmed_replies':1,'new_long_campaign_or_performance_claim':False,
            'not_refreshed':['Full Linux Node','Eight-country startup matrix','External old-save fixture matrix','Long campaign and Russia activation']},
        'research_counts':read(REPO/'docs/campaign-certification/C01/research-index.json')['counts'],
        'review_url':launch['url'],'evidence':{'inventory_sha256':sha(DEST/'evidence/inventory.json'),'logical_files':len(inventory['files']),'stored_bytes':inventory['stored_bytes']}}
    (DEST/'manifest.json').write_text(json.dumps(manifest,indent=2)+'\n',encoding='utf8')
    print(json.dumps(manifest,indent=2),flush=True)
if __name__=='__main__':main()
