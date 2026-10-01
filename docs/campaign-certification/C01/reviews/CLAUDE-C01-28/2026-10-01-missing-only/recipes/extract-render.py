import html, json, pathlib, re, subprocess
from html.parser import HTMLParser
from pypdf import PdfReader

W = pathlib.Path('D:/spheres-offload/codex-next-20260928/review-c01-28-20261001')
O = W.parent / 'ru28-originals-20261001'
(O/'texts').mkdir(exist_ok=True)
(O/'renders').mkdir(exist_ok=True)
class Text(HTMLParser):
    def __init__(self):
        super().__init__(); self.out=[]; self.skip=0
    def handle_starttag(self, tag, attrs):
        if tag in ('script', 'style'): self.skip+=1
        if tag in ('p','br','div','h1','h2','h3','li','tr'): self.out.append('\n')
    def handle_endtag(self, tag):
        if tag in ('script','style'): self.skip=max(0,self.skip-1)
        if tag in ('p','div','h1','h2','h3','li','tr'): self.out.append('\n')
    def handle_data(self, data):
        if not self.skip: self.out.append(data)
held=json.loads((W/'docs/campaign-certification/C01/integrations/CLAUDE-C01-28/source-holds.json').read_text())['sources']
retrieval=json.loads((O/'retrieval.json').read_text())
byid={s['source_id']:s for s in held}
for row in retrieval['sources']:
    if not row['exact']: continue
    sid=row['source_id']; b=pathlib.Path(row['body_path']); raw=b.read_bytes()
    if (O/'texts'/(sid+'.txt')).exists(): continue
    ex=json.loads((W/byid[sid]['extract']['path']).read_text(encoding='utf8'))
    if raw.startswith(b'%PDF'):
        pdf=PdfReader(b); text='\n'.join('=== PDF PAGE '+str(i+1)+' ===\n'+(p.extract_text() or '') for i,p in enumerate(pdf.pages))
        subprocess.run(['pdftoppm','-png','-r','130',str(b),str(O/'renders'/sid)],check=True,stdout=subprocess.DEVNULL)
        print(sid, 'PDF',len(pdf.pages),'pages',flush=True)
    else:
        if 'KOI8-R' in ex.get('provenance_note',''): codec='koi8-r'
        elif b'charset=utf-8' in raw.lower() or b'charset="utf-8' in raw.lower(): codec='utf-8'
        else: codec='cp1251'
        h=Text(); h.feed(raw.decode(codec)); text=re.sub(r'\n\s*\n', '\n', ''.join(h.out))
        print(sid,codec,flush=True)
    (O/'texts'/(sid+'.txt')).write_text(text,encoding='utf8')
    (O/'texts'/(sid+'.claims.json')).write_text(json.dumps(ex['rows'],ensure_ascii=False,indent=2)+'\n',encoding='utf8')
