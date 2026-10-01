"""Decode exact newly returned originals externally for independent passage review."""
import hashlib,json,pathlib,re
from html.parser import HTMLParser
HERE=pathlib.Path(__file__).resolve().parent
ROOT=HERE.parents[4]
class Text(HTMLParser):
 def __init__(self): super().__init__(convert_charrefs=True); self.parts=[]; self.hidden=0
 def handle_starttag(self,tag,attrs):
  if tag in ['script','style']: self.hidden+=1
  if tag in ['p','div','br','h1','h2','h3','h4','tr','li']: self.parts.append('\n')
 def handle_endtag(self,tag):
  if tag in ['script','style']: self.hidden-=1
  if tag in ['p','div','h1','h2','h3','h4','tr','li']: self.parts.append('\n')
 def handle_data(self,data):
  if self.hidden==0:self.parts.append(data)
sources={s['id']:s for s in json.loads((ROOT/'docs/campaign-certification/C01/research/japan.json').read_text(encoding='utf8'))['sources']}
manifest=[]
for row in json.loads((HERE/'retrieval.json').read_text(encoding='utf8'))['results']:
 if not row['exact']: continue
 raw=pathlib.Path(row['body_external']).read_bytes()
 assert len(raw)==row['expected_bytes'] and hashlib.sha256(raw).hexdigest()==row['expected_sha256']
 extract=json.loads((ROOT/sources[row['id']]['snapshot']['path']).read_text(encoding='utf8'))
 declaration=extract.get('source_character_encoding','utf8')
 encoding='cp932' if declaration=='Shift_JIS (cp932)' else declaration.split(' (',1)[0]
 parser=Text(); parser.feed(raw.decode(encoding))
 text='\n'.join(line.strip() for line in ''.join(parser.parts).splitlines() if line.strip())
 path=pathlib.Path(row['body_external']).with_suffix('.readable.txt')
 path.write_text(text,encoding='utf8',newline='\n')
 manifest.append({'id':row['id'],'original_external':row['body_external'],'original_sha256':row['sha256'],'encoding':encoding,'readable_external':str(path),'readable_bytes':len(path.read_bytes()),'readable_sha256':hashlib.sha256(path.read_bytes()).hexdigest()})
(HERE/'readable-manifest.json').write_text(json.dumps(manifest,ensure_ascii=False,indent=2)+'\n',encoding='utf8',newline='\n')
print('Decoded',len(manifest),'exact originals externally; historical decisions remain manual.')
