import datetime, hashlib, json, pathlib, subprocess, urllib.request

base=pathlib.Path(__file__).resolve().parent
repo=base/'integration'
out=base/'evidence/C01-tonga-review-20260921'
out.mkdir(exist_ok=True)
packet=json.loads(subprocess.check_output(['git','show','426f0dd:docs/campaign-certification/C01/research/tonga.json'],cwd=repo,text=True,encoding='utf8'))
sources={s['id']:s for s in packet['sources']}
checks=[('to_sc_kiu_v_tuionetoa_20220429','trial',[1,2,4,33,34],602620,'57fb2ac33421699958da3267e9d0540fa533ff2f944c36c022d5535aa3012372'),
        ('to_court_peoples_party_20220809','appeal',[9,12,16],307617,None)]
record={'reviewed_commit':'426f0ddb17a93c3d8b9a809c0129b441b29d3fe6','accessed_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'sources':[]}
for sid,name,pages,size,digest in checks:
    url=sources[sid]['url']
    req=urllib.request.Request(url,headers={'User-Agent':'Spheres-source-review/1.0'})
    with urllib.request.urlopen(req,timeout=30) as r: raw=r.read()
    actual=hashlib.sha256(raw).hexdigest()
    assert len(raw)==size,(name,len(raw))
    if digest: assert actual==digest,(name,actual)
    pdf=out/(name+'.pdf')
    assert not pdf.exists()
    pdf.write_bytes(raw)
    for page in pages:
        subprocess.run(['pdftoppm','-f',str(page),'-l',str(page),'-scale-to','1500','-singlefile','-png',str(pdf),str(out/(name+'-'+str(page)))],check=True,stdout=subprocess.DEVNULL,stderr=subprocess.PIPE)
    record['sources'].append({'id':sid,'url':url,'bytes':len(raw),'sha256':actual,'rendered_pages_one_based':pages,'original_response_checked_in':False})
(out/'source-fetch.json').write_text(json.dumps(record,indent=2)+'\n',encoding='utf8')
print(json.dumps(record,indent=2))
