"""Offline receipt integrity only; no source requests or content certification."""
import argparse,gzip,hashlib,json,subprocess
from pathlib import Path
HERE=Path(__file__).resolve().parent
def read(p):return json.loads(p.read_text(encoding='utf-8-sig'))
def verify(data,row):
    assert len(data)==row['bytes'],('byte count',row['path'])
    assert hashlib.sha256(data).hexdigest()==row['sha256'],('hash',row['path'])
def contained(root,relative):
    result=(root/relative).resolve()
    assert result.is_relative_to(root.resolve()),relative
    return result
def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--external-root',type=Path)
    parser.add_argument('--skip-external',action='store_true')
    args=parser.parse_args(); manifest=read(HERE/'manifest.json')
    expected={r['path'] for r in manifest['payload']}
    actual={p.relative_to(HERE).as_posix() for p in HERE.rglob('*') if p.is_file() and p.name!='manifest.json' and '__pycache__' not in p.parts}
    assert expected==actual,('payload inventory',expected^actual)
    for row in manifest['payload']:verify(contained(HERE,row['path']).read_bytes(),row)
    repo=next(p for p in HERE.parents if (p/'.git').exists())
    for row in manifest['reviewed_git_inputs']:
        verify(subprocess.check_output(['git','show',manifest['reviewed_revision']+':'+row['path']],cwd=repo),row)
    external=args.external_root or Path(manifest['original_external_root']); checked=0
    if not args.skip_external:
        for row in manifest['external_files']:
            verify(contained(external,row['path']).read_bytes(),row);checked+=1
        for a in read(HERE/'source-attempts.json'):
            assert a['http_status']==200 and a['matches_submitted_pin']
            sid=a['source_id'];body=(external/'attempt-01'/(sid+'.body')).read_bytes()
            assert len(body)==a['submitted_pin']['bytes'] and hashlib.sha256(body).hexdigest()==a['submitted_pin']['sha256']
            decoded=gzip.decompress(body) if body[:2]==b'\x1f\x8b' else body
            suffix='.decoded.pdf' if decoded.startswith(b'%PDF') else '.decoded.html'
            assert decoded==(external/'attempt-01'/(sid+suffix)).read_bytes()
    sources=read(HERE/'source-review.json');claims=read(HERE/'claim-review.json');holders=read(HERE/'holder-review.json')
    assert (len(sources),len(claims),len(holders))==(14,21,5)
    assert len({s['source_id'] for s in sources})==14 and all(s['original_read'] for s in sources)
    assert len({c['claim_id'] for c in claims})==21
    holderclaims={c for h in holders for c in h['claim_ids']}
    assert holderclaims=={c['claim_id'] for c in claims if c['decision']=='accepted_as_selected_holder_support'}
    assert all(h['from'] is None and h['until'] is None and h['unchanged_from_submission'] for h in holders)
    scope=read(HERE/'scope-review.json')
    assert scope['prior_fields_preserved_after_declared_appends_removed'] and scope['held']==0
    print(json.dumps({'payload_files':len(expected),'reviewed_git_inputs':len(manifest['reviewed_git_inputs']),
      'external_files_checked':checked,'external_skipped':args.skip_external,'originals':14,'claims':21,'holder_observations':5,
      'integrity_only':True,'source_content_decisions':'separate recorded independent reading','runtime_or_country_qualification':False},sort_keys=True))
if __name__=='__main__':main()
