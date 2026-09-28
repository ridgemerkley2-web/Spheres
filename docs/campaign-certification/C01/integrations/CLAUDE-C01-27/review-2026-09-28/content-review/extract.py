import hashlib,json,pathlib
from bs4 import BeautifulSoup
from pypdf import PdfReader
BASE=pathlib.Path('D:/spheres-offload/codex-next-20260928')
ROOT=BASE/'review-in27'
OUT=BASE/'in27-content-review'
INDEX=json.loads((BASE/'in27-source-preparation/combined-index.json').read_text(encoding='utf-8'))
DATA=json.loads((ROOT/'docs/campaign-certification/C01/research/india.json').read_text(encoding='utf-8'))
records=[]
for source in DATA['sources']:
    if source['id'] not in INDEX: continue
    rec=INDEX[source['id']]
    body=pathlib.Path(rec.get('body_absolute',str(BASE/'in27-source-preparation'/rec['body'])))
    raw=body.read_bytes()
    assert len(raw)==rec['bytes'] and hashlib.sha256(raw).hexdigest()==rec['sha256'] and rec['exact']
    extract=json.loads((ROOT/source['snapshot']['path']).read_text(encoding='utf-8'))
    pages=extract.get('visual_review',{}).get('pdf_pages_one_based',[])
    if raw.startswith(b'%PDF'):
        reader=PdfReader(body)
        txt='\n'.join(f'\n--- PDF PAGE {n} ---\n'+reader.pages[n-1].extract_text() for n in pages)
    else:
        txt=raw.decode('utf-8',errors='replace')
        if txt.startswith('{'):
            v=json.loads(txt)
            if isinstance(v,dict): txt=v.get('content',{}).get('rendered',txt)
        soup=BeautifulSoup(txt,'html.parser')
        for node in soup(['script','style']): node.decompose()
        main=soup.select_one('[itemprop=articleBody], .article-content, .item-page') or soup
        txt=main.get_text(' ',strip=True)
    (OUT/'text'/f'{source["id"]}.txt').write_text(txt,encoding='utf-8')
    records.append({'id':source['id'],'body':str(body),'bytes':len(raw),'sha256':rec['sha256'],'pdf_pages':pages,'claims':source['claims']})
(OUT/'all-inputs.json').write_text(json.dumps(records,indent=2,ensure_ascii=False)+'\n',encoding='utf-8')
print(len(records),'exact bodies;',sum(len(r['claims']) for r in records),'claims')
