"""Sequential missing-only C01-29 retrieval; preserve every attempt and prior success."""
import argparse, datetime, hashlib, json, subprocess, time
from pathlib import Path

parser=argparse.ArgumentParser(description=__doc__)
parser.add_argument('--attempt',required=True)
parser.add_argument('--prior')
args=parser.parse_args()
ROOT=Path('D:/spheres-offload/codex-next-20260928/review-jp29-resumed')
BASE=Path(__file__).resolve().parent
OLD=Path('C:/Users/ridge/Documents/Codex/2026-09-05/pick-up-the-spheres-game-on/work/campaign-certification/evidence/jp29-source-review')
old=json.loads((OLD/'retrieval.json').read_text(encoding='utf8'))
packet=json.loads((ROOT/'docs/campaign-certification/C01/research/japan.json').read_text(encoding='utf8'))
sources={s['id']:s for s in packet['sources']}
missing=[s for s in old['sources'] if not s['exact']]
assert len(missing)==19
for row in old['sources']:
    if row['exact']:
        data=(OLD/row['body']).read_bytes()
        assert len(data)==row['bytes'] and hashlib.sha256(data).hexdigest()==row['sha256']
done=set()
if args.prior:done={r['id'] for r in json.loads(Path(args.prior).read_text())['results'] if r['exact']}
out=BASE/args.attempt;out.mkdir(exist_ok=False);(out/'bodies').mkdir();(out/'headers').mkdir()
results=[]
for source in missing:
    sid=source['id']
    if sid in done:continue
    declared=sources[sid];extract=json.loads((ROOT/declared['snapshot']['path']).read_text(encoding='utf8'))
    assert source['url']==declared['url'] and source['expected_sha256']==extract['source_response_sha256']
    body=out/'bodies'/(sid+'.bin');headers=out/'headers'/(sid+'.txt')
    command=['curl.exe','--silent','--show-error','--connect-timeout','15','--max-time','60','-H','Accept-Encoding: identity',
             '--output',str(body),'--dump-header',str(headers),'--write-out','%{json}',source['url']]
    start=datetime.datetime.now(datetime.timezone.utc).isoformat();t=time.monotonic()
    result=subprocess.run(command,capture_output=True);data=body.read_bytes() if body.exists() else b''
    meta=json.loads(result.stdout.decode('utf8')) if result.stdout else {}
    row={'id':sid,'url':source['url'],'start':start,'finish':datetime.datetime.now(datetime.timezone.utc).isoformat(),
      'elapsed_seconds':time.monotonic()-t,'command':command,'exit_code':result.returncode,'curl':meta,
      'stderr':result.stderr.decode('utf8',errors='replace'),'body':str(body),'headers':str(headers),'bytes':len(data),
      'sha256':hashlib.sha256(data).hexdigest(),'expected_bytes':source['expected_bytes'],'expected_sha256':source['expected_sha256']}
    row['exact']=result.returncode==0 and meta.get('http_code')==200 and len(data)==row['expected_bytes'] and row['sha256']==row['expected_sha256']
    results.append(row)
    with (out/'attempts.jsonl').open('a',encoding='utf8',newline='\n') as f:f.write(json.dumps(row,ensure_ascii=False)+'\n')
    print(sid,'EXACT' if row['exact'] else 'FAILED',meta.get('http_code'),len(data),flush=True)
    time.sleep(0.5)
receipt={'reviewed_submission':'3b30478562ec78c2392d0472061e773e23cb25dd','previous_correction':'c98d1d5668870e184c320a3e03f477bd7f89a12c',
 'reviewer':'Codex /root/review_gap_submission','previous_receipt':str(OLD/'retrieval.json'),
 'previous_35_bodies_rehashed':True,'skipped_this_attempt':sorted(done),'results':results,
 'exact':sum(r['exact'] for r in results),'failed':sum(not r['exact'] for r in results)}
(out/'retrieval.json').write_text(json.dumps(receipt,ensure_ascii=False,indent=2)+'\n',encoding='utf8',newline='\n')
print(json.dumps({'exact':receipt['exact'],'failed':receipt['failed']}),flush=True)
