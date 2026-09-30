import json,pathlib,urllib.request,urllib.error
BASE=pathlib.Path(__file__).resolve().parent
def capture(url,name):
    try:
        with urllib.request.urlopen(urllib.request.Request(url,headers={'User-Agent':'Spheres-evidence-review'}),timeout=60) as r:raw=r.read()
    except urllib.error.HTTPError as e:
        raw=e.read()
        with (BASE/(name+'.error')).open('xb') as f:f.write(raw)
        return {'http_status':e.code}
    with (BASE/name).open('xb') as f:f.write(raw)
    return json.loads(raw)
jobs=json.loads((BASE/'jobs.json').read_text())['jobs']
india=next(j for j in jobs if j['name']=='cell (india-1990)')
check=capture(india['check_run_url'],'india-1990-check-run.json')
if 'output' in check:
    annotations=capture(check['output']['annotations_url']+'?per_page=100','india-1990-annotations.json')
    print(json.dumps({'india_check':check.get('conclusion'),'annotations':annotations},indent=2))
for run,tag in [(36477494166,'ci032'),(36490073575,'ci440')]:
    a=capture('https://api.github.com/repos/ridgemerkley2-web/Spheres/actions/runs/'+str(run),tag+'-run.json')
    b=capture('https://api.github.com/repos/ridgemerkley2-web/Spheres/actions/runs/'+str(run)+'/jobs?per_page=100',tag+'-jobs.json')
    print(json.dumps({'run':run,'conclusion':a.get('conclusion'),'jobs':[{k:j.get(k) for k in ['id','name','conclusion']} for j in b.get('jobs',[])]}))
