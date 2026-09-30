"""Retain exact GitHub run metadata and artifacts; auth stays on api.github.com."""
import concurrent.futures, hashlib, json, os, pathlib, subprocess, urllib.error, urllib.parse, urllib.request

BASE=pathlib.Path(__file__).resolve().parent
OUT=BASE/'final-collection-20260930-01'
REPO='ridgemerkley2-web/Spheres'
RUN=36474011141
class NoRedirect(urllib.request.HTTPRedirectHandler):
    def redirect_request(self, req, fp, code, msg, headers, newurl):
        return None

def new_json(p,v):
    with p.open('x',encoding='utf-8',newline='\n') as f:json.dump(v,f,indent=2);f.write('\n')

def main():
    OUT.mkdir(exist_ok=False)
    (OUT/'downloads').mkdir(); (OUT/'metadata').mkdir(); (OUT/'job-logs').mkdir()
    cred=subprocess.run(['git','-c','credential.interactive=never','credential','fill'],input='protocol=https\nhost=github.com\npath='+REPO+'.git\n\n',text=True,capture_output=True,env={**os.environ,'GIT_TERMINAL_PROMPT':'0'},check=True)
    auth=dict(x.split('=',1) for x in cred.stdout.splitlines() if '=' in x)
    token=auth['password']
    opener=urllib.request.build_opener(NoRedirect())
    def api(path):
        req=urllib.request.Request('https://api.github.com/repos/'+REPO+path,headers={'Authorization':'Bearer '+token,'Accept':'application/vnd.github+json','User-Agent':'Spheres-evidence-collection'})
        return opener.open(req,timeout=120)
    def raw(path,name):
        with api(path) as response: data=response.read()
        with (OUT/name).open('xb') as f:f.write(data)
        return json.loads(data)
    run=raw('/actions/runs/'+str(RUN),'run.json')
    jobs=raw('/actions/runs/'+str(RUN)+'/jobs?per_page=100','jobs.json')['jobs']
    artifacts=raw('/actions/runs/'+str(RUN)+'/artifacts?per_page=100','artifacts.json')['artifacts']
    assert run['id']==RUN and run['run_attempt']==1 and run['head_sha']=='5d970f6d7370baf16760585c641d81253d1c2175' and run['status']=='completed'
    plan=json.loads((BASE/'frozen/plan.json').read_text())
    items=[]
    for cell in plan['cells']:
        name=cell['id']; job=next(j for j in jobs if j['name']=='cell ('+name+')')
        assert job['status']=='completed'
        new_json(OUT/'metadata'/(name+'.job.json'),job)
        matches=[a for a in artifacts if a['name']=='full-stability-cell-'+name+'-1']
        assert len(matches)<=1
        if matches:new_json(OUT/'metadata'/(name+'.artifact.json'),matches[0])
        items.append((name,job,matches[0] if matches else None))
    def redirected(path,destination):
        try: response=api(path)
        except urllib.error.HTTPError as e:
            if e.code not in (301,302,303,307,308):raise
            url=e.headers['Location']; target=urllib.parse.urlparse(url)
            assert target.scheme=='https' and target.hostname and (target.hostname.endswith('.blob.core.windows.net') or target.hostname.endswith('.githubusercontent.com'))
            # A new request intentionally carries NO GitHub credentials.
            response=urllib.request.urlopen(url,timeout=180)
        h=hashlib.sha256(); size=0
        with response,destination.open('xb') as f:
            while b:=response.read(1048576):f.write(b);h.update(b);size+=len(b)
            f.flush();os.fsync(f.fileno())
        return {'bytes':size,'sha256':h.hexdigest()}
    def collect(item):
        name,job,artifact=item; result={'cell':name,'job_id':job['id'],'job_conclusion':job['conclusion'],'artifact_available':artifact is not None}
        try: result['job_log']=redirected('/actions/jobs/'+str(job['id'])+'/logs',OUT/'job-logs'/(name+'.log'))
        except urllib.error.HTTPError as e:
            result['log_http_status']=e.code
            with (OUT/'job-logs'/(name+'.error.txt')).open('xb') as f:f.write(e.read())
        except Exception as e: result['log_error']=type(e).__name__+': '+str(e)
        if artifact:
            try:
                result['artifact_id']=artifact['id']; result['download']=redirected('/actions/artifacts/'+str(artifact['id'])+'/zip',OUT/'downloads'/(name+'.zip'))
                assert result['download']=={'bytes':artifact['size_in_bytes'],'sha256':artifact['digest'][7:]}
                result['artifact_verified']=True
            except urllib.error.HTTPError as e:
                result['artifact_http_status']=e.code
                with (OUT/'downloads'/(name+'.error.txt')).open('xb') as f:f.write(e.read())
            except Exception as e:result['artifact_error']=type(e).__name__+': '+str(e)
        new_json(OUT/'metadata'/(name+'.download.json'),result)
        print(json.dumps(result),flush=True)
        return result
    with concurrent.futures.ThreadPoolExecutor(max_workers=4) as pool:results=list(pool.map(collect,items))
    new_json(OUT/'download-results.json',{'run_id':RUN,'qualification':False,'full_matrix_passed':False,'cells':results})

if __name__=='__main__':main()
