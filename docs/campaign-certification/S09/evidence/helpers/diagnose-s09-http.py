import concurrent.futures,datetime,hashlib,http.client,json,pathlib,re,socket,subprocess,time
b=pathlib.Path(__file__).resolve().parent;out=b/'evidence/S09-startup-http';out.mkdir(exist_ok=False);run=out/'server';run.mkdir()
exe=b/'integration-target/release/spheres-web.exe'
with socket.socket() as s:s.bind(('127.0.0.1',0));port=s.getsockname()[1]
with (out/'server.log').open('wb') as log:
 p=subprocess.Popen([str(exe),'--port',str(port),'--no-open'],cwd=run,stdout=log,stderr=log,creationflags=subprocess.CREATE_NO_WINDOW)
 result={'pid':p.pid,'port':port,'started_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'routes':[]}
 def request(route):
  c=http.client.HTTPConnection('127.0.0.1',port,timeout=10);t=time.perf_counter()
  try:c.request('GET',route);r=c.getresponse();data=r.read();return {'route':route,'status':r.status,'bytes':len(data),'seconds':time.perf_counter()-t},data
  finally:c.close()
 try:
  for i in range(50):
   try:meta,body=request('/api/build');break
   except OSError:time.sleep(.1)
  else:raise RuntimeError('server startup')
  result['build']=json.loads(body);assert result['build']['revision']=='94d2c094b2c6'
  meta,body=request('/');result['root']=meta
  routes=list(dict.fromkeys(re.findall(r'<(?:script[^>]*src|link[^>]*href)="([^\"]+)"',body.decode())))
  assert all(r.startswith('/') for r in routes)
  with concurrent.futures.ThreadPoolExecutor(max_workers=6) as pool:
   for meta,body in pool.map(request,routes):
    result['routes'].append(meta);assert meta['status']==200
  result['passed']=True
 except BaseException as e:result['passed']=False;result['error']=repr(e)
 finally:
  p.terminate();p.wait(timeout=10);result['finished_utc']=datetime.datetime.now(datetime.timezone.utc).isoformat()
  (out/'result.json').write_text(json.dumps(result,indent=2)+'\n',encoding='utf-8')
  print(json.dumps({'passed':result['passed'],'routes':len(result['routes']),'max_seconds':max((r['seconds'] for r in result['routes']),default=0),'result':str(out/'result.json')}),flush=True)
