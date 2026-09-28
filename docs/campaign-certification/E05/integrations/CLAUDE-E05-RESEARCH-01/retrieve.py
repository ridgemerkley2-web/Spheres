import concurrent.futures,datetime,hashlib,html,io,json,pathlib,re,urllib.request,urllib.error,zipfile
from pypdf import PdfReader
BASE=pathlib.Path(__file__).resolve().parent
REPO=BASE.parent.parent/'c01-source-followups-review'
DATA=REPO/'docs/research/company-pilot'
(BASE/'bodies').mkdir(exist_ok=True);(BASE/'text').mkdir(exist_ok=True)
sources=json.loads((DATA/'sources.json').read_text(encoding='utf-8'))['sources']
claims=[c for p in sorted((DATA/'dossiers').glob('*.json')) for c in json.loads(p.read_text(encoding='utf-8'))['claims']]
def clean(s):return re.sub(r'\s+','',html.unescape(s)).casefold()
def retrieve(source):
    sid=source['id']; r={'source':sid,'url':source['url'],'started_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'method':'ordinary urllib default User-Agent GET, Accept-Encoding identity, no cookies or bypass, maximum64MiB body'}
    try:
        req=urllib.request.Request(source['url'],headers={'Accept-Encoding':'identity'})
        with urllib.request.urlopen(req,timeout=35) as response:
            data=response.read(64*1024*1024+1)
            r.update(status=response.status,final_url=response.url,headers=dict(response.headers))
    except urllib.error.HTTPError as error:
        data=error.read(1*1024*1024);r.update(status=error.code,headers=dict(error.headers),error=str(error))
    except Exception as error:
        data=b'';r.update(status=None,error=repr(error))
    r.update(completed_utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),bytes=len(data),sha256=hashlib.sha256(data).hexdigest())
    r['matches_first_response']=r['status']==200 and len(data)==source['response']['bytes'] and r['sha256']==source['response']['sha256']
    r['matches_reported_recheck']=r['status']==200 and len(data)==source['byte_stability']['recheck_bytes'] and r['sha256']==source['byte_stability']['recheck_sha256']
    ext='.pdf' if data.startswith(b'%PDF') else '.zip' if data.startswith(b'PK') else '.body'
    path=BASE/'bodies'/(sid+ext);path.write_bytes(data);r['body']=str(path.relative_to(BASE))
    text=''; pages={}
    own=[c for c in claims if c['source']==sid]
    try:
        if len(data)>64*1024*1024: raise ValueError('body exceeded64MiB; not treated as complete')
        if r['status']==200 and ext=='.pdf':
            pdf=PdfReader(io.BytesIO(data));r['pdf_pages']=len(pdf.pages)
            # The authored locator is one physical PDF page; extract only cited pages.
            for n in sorted({int(c['locator']['page']) for c in own if isinstance(c['locator'].get('page'),int)}):
                pages[n]=pdf.pages[n-1].extract_text() or ''
            text='\n'.join(pages.values())
        elif r['status']==200 and ext=='.zip':
            archive=zipfile.ZipFile(io.BytesIO(data))
            files=[f for f in archive.infolist() if f.filename.endswith(('.xhtml','.html','.htm'))]
            if sum(f.file_size for f in files)>128*1024*1024: raise ValueError('XHTML payload exceeds128MiB')
            text='\n'.join(archive.read(f).decode('utf-8','replace') for f in files)
            text=re.sub('<[^>]+>',' ',text)
        elif r['status']==200:
            charset=re.search(r'charset=[\"\x27]?([^\s;\"\x27]+)',str(r['headers'].get('Content-Type','')),re.I)
            text=data.decode(charset.group(1) if charset else 'utf-8','replace')
            text=re.sub('<[^>]+>',' ',text)
        text=html.unescape(text)
        (BASE/'text'/(sid+'.txt')).write_text(text,encoding='utf-8')
        r['anchors']=[{'claim':c['id'],'anchor':c['anchor'],'locator':c['locator'],'found':bool(text) and clean(c['anchor']) in clean(pages.get(c['locator'].get('page'),text))} for c in own]
    except Exception as error:r['extraction_error']=repr(error)
    (BASE/(sid+'.json')).write_text(json.dumps(r,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    print(sid,'status',r['status'],'exact',r['matches_first_response'],'anchors',sum(a['found'] for a in r.get('anchors',[])),len(own),flush=True)
    return r
with concurrent.futures.ThreadPoolExecutor(max_workers=4) as pool:results=list(pool.map(retrieve,sources))
(BASE/'retrievals.json').write_text(json.dumps({'reviewer':'Codex /root/review_source05','pypdf':'6.17.0','results':results},ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
print('DONE',len(results),sum(r['status']==200 for r in results),'HTTP200',sum(r['matches_first_response'] for r in results),'exactfirst')
