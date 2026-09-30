import base64,hashlib,json,pathlib,subprocess
from pypdf import PdfReader
W=pathlib.Path('D:/spheres-offload/codex-next-20260928/review-su35');E=W.parent/'su35-review';(E/'pages').mkdir(exist_ok=False);(E/'renders').mkdir(exist_ok=False)
data=json.loads((W/'docs/campaign-certification/C01/research/ussr.json').read_text(encoding='utf8'));retrieved=json.loads((E/'retrieval.json').read_text())['sources'];ids={r['source_id'] for r in retrieved};sources=[s for s in data['sources'] if s['id'] in ids];out=[];alltext=[]
for s in sources:
 x=json.loads((W/s['snapshot']['path']).read_text(encoding='utf8'));r=next(r for r in retrieved if r['source_id']==s['id']);raw=pathlib.Path(r['body_path']).read_bytes();assert r['exact'] and hashlib.sha256(raw).hexdigest()==x['source_response_sha256'];reader=PdfReader(r['body_path']);pages=x['visual_review']['pdf_pages_one_based']+x['visual_review'].get('facsimile_pages_one_based',[]);texts=[]
 for n in pages:
  text=reader.pages[n-1].extract_text(extraction_mode='layout');texts.append({'page':n,'text':text});(E/'pages'/f'{s["id"]}__{n}.txt').write_text(text,encoding='utf8')
  subprocess.run(['pdftoppm','-f',str(n),'-l',str(n),'-singlefile','-scale-to','1500','-png',r['body_path'],str(E/'renders'/f'{s["id"]}__{n}')],check=True,stdout=subprocess.DEVNULL,stderr=subprocess.PIPE)
 sb=(W/s['snapshot']['path']).read_bytes();assert len(sb)==s['snapshot']['bytes'] and hashlib.sha256(sb).hexdigest()==s['snapshot']['sha256']
 row={'source_id':s['id'],'source_url':s['url'],'original_url':s.get('original_url'),'source_type':s['source_type'],'bytes':len(raw),'sha256':hashlib.sha256(raw).hexdigest(),'sha1_base32':base64.b32encode(hashlib.sha1(raw).digest()).decode(),'snapshot':s['snapshot'],'pages':pages,'claims':s['claims']};out.append(row)
 alltext.append('\nSOURCE '+s['id']+'\n'+json.dumps(s['claims'],ensure_ascii=False,indent=2)+'\n'+'\n'.join('PDF PAGE '+str(t['page'])+'\n'+t['text'] for t in texts));print(s['id'],len(pages),flush=True)
(E/'source-pins.json').write_text(json.dumps(out,ensure_ascii=False,indent=2)+'\n',encoding='utf8');(E/'review-content.txt').write_text('\n'.join(alltext),encoding='utf8')
print('SOURCES',len(out),'PAGES',sum(len(s['pages']) for s in out))
