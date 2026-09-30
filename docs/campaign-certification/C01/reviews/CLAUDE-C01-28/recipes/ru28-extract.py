import pathlib,json,hashlib,base64,bs4
from pypdf import PdfReader
W=pathlib.Path('D:/spheres-offload/codex-next-20260928/review-ru28');E=W.parent/'ru28-review';(E/'texts').mkdir(exist_ok=True)
ss=json.loads((E/'new-sources.json').read_text(encoding='utf8'))
rows=[]
for name in ['retrieval.json','retrieval-retry.json']:rows+=json.loads((E/name).read_text())['sources']
passed={r['source_id']:r for r in rows if r['exact']};out=[]
for s in ss:
    ex=json.loads((W/s['snapshot']['path']).read_text(encoding='utf8'));b=(W/s['snapshot']['path']).read_bytes()
    assert len(b)==s['snapshot']['bytes'] and hashlib.sha256(b).hexdigest()==s['snapshot']['sha256']
    v={'source_id':s['id'],'source_type':s['source_type'],'url':s['url'],'extract':s['snapshot'],
       'expected_bytes':ex['source_response_bytes'],'expected_sha256':ex['source_response_sha256'],
       'expected_sha1_base32':ex['source_response_sha1_base32'],'claims':[c['id'] for c in s['claims']],
       'reproduced':s['id'] in passed}
    if s['id'] in passed:
        r=passed[s['id']];b=pathlib.Path(r['body_path']).read_bytes();sha1=base64.b32encode(hashlib.sha1(b).digest()).decode()
        assert sha1==ex['source_response_sha1_base32']==ex['archive_index_digest']
        if b.startswith(b'%PDF'):
            reader=PdfReader(r['body_path']); text='\n'.join('\nPDF PAGE '+str(i+1)+'\n'+(page.extract_text() or '') for i,page in enumerate(reader.pages)); enc='PDF';v['pdf_pages']=len(reader.pages)
        else:
            soup=bs4.BeautifulSoup(b,'html.parser');enc=soup.original_encoding
            for tag in soup(['script','style','noscript']):tag.decompose()
            main=soup.find('main') or soup.find('article') or soup
            text=main.get_text('\n',strip=True)
        f=E/'texts'/(s['id']+'.txt');f.write_text(text,encoding='utf8');v.update(actual_sha1_base32=sha1,decoder=enc,text_file=str(f),body_path=r['body_path'])
    out.append(v)
(E/'source-verification.json').write_text(json.dumps(out,ensure_ascii=False,indent=2)+'\n',encoding='utf8')
print('exact',len(passed),'missing',len(ss)-len(passed))
