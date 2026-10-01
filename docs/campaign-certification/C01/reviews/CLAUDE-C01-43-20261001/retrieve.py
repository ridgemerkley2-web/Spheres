"""Run only in assigned Archive slot; one sequential pass, no redirects/retries."""
import datetime as dt,gzip,hashlib,json,re,subprocess,time
from html.parser import HTMLParser
from pathlib import Path
ROOT=Path('D:/spheres-offload/codex-next-20260928/review-c01-43-20261001')
OUT=Path(__file__).parent/'attempt-01'
def stamp():return dt.datetime.now(dt.timezone.utc).isoformat()
def pin(p):
 b=p.read_bytes();return {'path':str(p),'bytes':len(b),'sha256':hashlib.sha256(b).hexdigest()}
class Text(HTMLParser):
 def __init__(self):super().__init__(convert_charrefs=True);self.skip=0;self.parts=[]
 def handle_starttag(self,t,a):
  if t in ('script','style'):self.skip+=1
  if t in ('p','div','h1','h2','h3','li','br','tr','td'):self.parts.append('\n')
 def handle_endtag(self,t):
  if t in ('script','style') and self.skip:self.skip-=1
  if t in ('p','div','h1','h2','h3','li','tr','td'):self.parts.append('\n')
 def handle_data(self,d):
  if not self.skip:self.parts.append(d)
def main():
 OUT.mkdir(exist_ok=False)
 p=json.loads((ROOT/'docs/campaign-certification/C01/research/brazil.json').read_text(encoding='utf-8'))
 records=[];stop=False
 for i,s in enumerate(p['sources'][248:]):
  sid=s['id']
  if stop:records.append({'source_id':sid,'url':s['url'],'status':'not_attempted_after_block_or_failed_response'});continue
  if i:time.sleep(15)
  body=OUT/(sid+'.body');headers=OUT/(sid+'.headers')
  cmd=['curl.exe','--silent','--show-error','--max-time','45','--retry','0','--header','Accept-Encoding: identity',
       '--dump-header',str(headers),'--output',str(body),'--write-out','%{json}',s['url']]
  start=stamp();r=subprocess.run(cmd,capture_output=True)
  std=OUT/(sid+'.curl.stdout');err=OUT/(sid+'.curl.stderr');std.write_bytes(r.stdout);err.write_bytes(r.stderr)
  try:meta=json.loads(r.stdout)
  except (ValueError,UnicodeError):meta={}
  e=json.loads((ROOT/s['snapshot']['path']).read_text(encoding='utf-8'))
  row={'source_id':sid,'url':s['url'],'started_utc':start,'finished_utc':stamp(),'command':cmd,'exit_code':r.returncode,
       'http_status':meta.get('http_code'),'curl':meta,'files':[pin(f) for f in (body,headers,std,err) if f.exists()],
       'submitted_pin':{'bytes':e['source_response_bytes'],'sha256':e['source_response_sha256']}}
  if body.exists():
   b=body.read_bytes();row['matches_submitted_pin']=len(b)==e['source_response_bytes'] and hashlib.sha256(b).hexdigest()==e['source_response_sha256']
   if meta.get('http_code')==200:
    decoded=gzip.decompress(b) if b[:2]==b'\x1f\x8b' else b
    if 'decoded_response_sha256' in e:
     row['decoded_matches_submitted_pin']=len(decoded)==e['decoded_response_bytes'] and hashlib.sha256(decoded).hexdigest()==e['decoded_response_sha256']
    is_pdf=decoded.startswith(b'%PDF');dp=OUT/(sid+('.decoded.pdf' if is_pdf else '.decoded.html'));dp.write_bytes(decoded);row['files'].append(pin(dp))
    if not is_pdf:
     m=re.search(rb'charset\s*=\s*["\']?([A-Za-z0-9_-]+)',decoded[:15000],re.I)
     codec=m.group(1).decode('ascii') if m else 'utf-8';row['html_charset']=codec
     text=Text();text.feed(decoded.decode(codec,errors='replace'))
     tp=OUT/(sid+'.text.txt');tp.write_text('\n'.join(x.strip() for x in ''.join(text.parts).splitlines() if x.strip())+'\n',encoding='utf-8',newline='\n');row['files'].append(pin(tp))
  records.append(row);(OUT/'attempts.json').write_text(json.dumps(records,ensure_ascii=False,indent=2)+'\n',encoding='utf-8',newline='\n')
  print(sid,row['http_status'],row.get('matches_submitted_pin'),flush=True)
  stop=bool(r.returncode or row['http_status']!=200)
 (OUT/'attempts.json').write_text(json.dumps(records,ensure_ascii=False,indent=2)+'\n',encoding='utf-8',newline='\n')
if __name__=='__main__':main()
