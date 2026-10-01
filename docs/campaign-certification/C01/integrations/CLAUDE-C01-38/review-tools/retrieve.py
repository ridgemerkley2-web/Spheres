import concurrent.futures, datetime, hashlib, html, io, json, pathlib, subprocess, tarfile
from html.parser import HTMLParser
ROOT=pathlib.Path(r'D:\spheres-offload\codex-next-20260928\review-c01-38-20261001')
OUT=pathlib.Path(__file__).parent
BASE='f3e18e8306a0a7b1098b91f53f00efbb30da7990'
PACKET='docs/campaign-certification/C01/research/france.json'
old=json.loads(subprocess.check_output(['git','show',BASE+':'+PACKET],cwd=ROOT))
new=json.loads((ROOT/PACKET).read_text(encoding='utf-8'))
oldids={s['id'] for s in old['sources']}
sources=[s for s in new['sources'] if s['id'] not in oldids]
assert len(sources)==22 and sum(len(s['claims']) for s in sources)==33
def sha(b): return hashlib.sha256(b).hexdigest()
class Text(HTMLParser):
    def __init__(self): super().__init__(); self.skip=0; self.out=[]
    def handle_starttag(self,tag,attrs):
        if tag in ('script','style'): self.skip+=1
        if tag in ('p','div','h1','h2','br','article','li'): self.out.append('\n')
    def handle_endtag(self,tag):
        if tag in ('script','style'): self.skip=max(0,self.skip-1)
    def handle_data(self,data):
        if not self.skip:self.out.append(data)
def textbody(body):
    parser=Text(); parser.feed(body.decode('utf-8',errors='replace'))
    return '\n'.join(' '.join(x.split()) for x in ''.join(parser.out).splitlines() if x.strip())
def run(source,attempt_numbers=(1,2)):
    sid=source['id']; derived=json.loads((ROOT/source['snapshot']['path']).read_text(encoding='utf-8'))
    result={'source_id':sid,'url':source['url'],'claims':[c['id'] for c in source['claims']],
      'expected_bytes':derived['source_response_bytes'],'expected_sha256':derived['source_response_sha256'],'attempts':[]}
    for attempt in attempt_numbers:
        folder=OUT/'responses'/sid/str(attempt); folder.mkdir(parents=True,exist_ok=False)
        body=folder/'body'; headers=folder/'headers.txt'; errors=folder/'curl-stderr.txt'
        args=['curl.exe','--location','--max-time','50','--connect-timeout','15','--silent','--show-error','--fail',
              '--dump-header',str(headers),'--output',str(body),'--write-out','%{http_code}\n%{url_effective}\n',source['url']]
        proc=subprocess.run(args,capture_output=True); errors.write_bytes(proc.stderr)
        record={'attempt':attempt,'at_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),
          'command':args,'exit_code':proc.returncode,'response':proc.stdout.decode(errors='replace'),
          'body_path':str(body),'headers_path':str(headers),'error_path':str(errors)}
        if body.exists():
            raw=body.read_bytes(); record.update(bytes=len(raw),sha256=sha(raw))
            record['exact']=proc.returncode==0 and len(raw)==result['expected_bytes'] and sha(raw)==result['expected_sha256']
        result['attempts'].append(record)
        if record.get('exact'):
            result['exact']=True; result['selected_attempt']=attempt
            textdir=OUT/'source-text'; textdir.mkdir(exist_ok=True)
            if source['url'].endswith('.tar.gz'):
                members=sorted({row['locator'][key] for row in derived['rows'] for key in ('archive_member','article_member') if key in row['locator']})
                with tarfile.open(fileobj=io.BytesIO(raw),mode='r:gz') as archive:
                    result['members']=[]
                    chunks=[]
                    for member in members:
                        value=archive.extractfile(member).read(); target=folder/pathlib.PurePosixPath(member).name;target.write_bytes(value)
                        result['members'].append({'member':member,'bytes':len(value),'sha256':sha(value),'path':str(target)})
                        chunks.append('MEMBER '+member+'\n'+value.decode('utf-8'))
                rendered='\n\n'.join(chunks)
            else: rendered=textbody(raw)
            target=textdir/(sid+'.txt');target.write_text(rendered,encoding='utf-8')
            result['text_path']=str(target);result['text_sha256']=sha(target.read_bytes())
            break
    result.setdefault('exact',False)
    print(sid, 'EXACT' if result['exact'] else 'UNRESOLVED',flush=True)
    return result
if __name__=='__main__':
    with concurrent.futures.ThreadPoolExecutor(max_workers=3) as pool: results=list(pool.map(run,sources))
    (OUT/'source-verification.json').write_text(json.dumps({'format':'spheres-independent-source-retrieval/v1','reviewer':'Codex /root',
     'submission':'acf33f09a5d28cbc9bf9f539e152e9846a7bf395','started_from':BASE,'scope':'Byte retrieval only; content decisions are separate',
     'sources':results,'exact':sum(r['exact'] for r in results),'total':len(results)},ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    print(json.dumps({'exact':sum(r['exact'] for r in results),'total':len(results)}),flush=True)
