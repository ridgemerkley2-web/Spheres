"""Offline evidence-integrity check; never performs source retrieval or certifies content."""
import argparse
import gzip
import hashlib
import json
import subprocess
from pathlib import Path

HERE=Path(__file__).resolve().parent

def read(path):return json.loads(path.read_text(encoding='utf-8'))
def verify(data,row):
    assert len(data)==row['bytes'], ('byte count',row['path'])
    assert hashlib.sha256(data).hexdigest()==row['sha256'], ('sha256',row['path'])
def contained(root,relative):
    path=(root/relative).resolve()
    assert path.is_relative_to(root.resolve()), relative
    return path

def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--external-root',type=Path)
    parser.add_argument('--skip-external',action='store_true')
    args=parser.parse_args()
    manifest=read(HERE/'manifest.json')
    expected={r['path'] for r in manifest['payload']}
    actual={p.relative_to(HERE).as_posix() for p in HERE.rglob('*') if p.is_file() and p.name!='manifest.json' and '__pycache__' not in p.parts}
    assert actual==expected, ('payload inventory',actual^expected)
    for row in manifest['payload']:verify(contained(HERE,row['path']).read_bytes(),row)
    repo=next(parent for parent in HERE.parents if (parent/'.git').exists())
    for row in manifest['reviewed_git_inputs']:
        data=subprocess.check_output(['git','show',manifest['reviewed_revision']+':'+row['path']],cwd=repo)
        verify(data,row)
    external=args.external_root or Path(manifest['original_external_root'])
    checked=0
    if not args.skip_external:
        for row in manifest['external_files']:
            verify(contained(external,row['path']).read_bytes(),row);checked+=1
        for attempt in read(HERE/'source-attempts.json'):
            assert attempt['http_status']==200 and attempt['matches_submitted_pin']
            sid=attempt['source_id'];body=(external/'attempt-01'/(sid+'.body')).read_bytes()
            submitted=attempt['submitted_pin']
            assert len(body)==submitted['bytes'] and hashlib.sha256(body).hexdigest()==submitted['sha256']
            decoded=gzip.decompress(body) if body[:2]==b'\x1f\x8b' else body
            assert decoded==(external/'attempt-01'/(sid+'.decoded.html')).read_bytes()
    sources=read(HERE/'source-review.json');claims=read(HERE/'claim-review.json');holders=read(HERE/'holder-review.json')
    assert (len(sources),len(claims),len(holders))==(7,27,18)
    assert len({r['source_id'] for r in sources})==7 and len({r['claim_id'] for r in claims})==27
    ids={r['claim_id'] for r in claims}
    assert all(set(h['claim_ids'])<=ids for h in holders)
    assert all(h['from'] is None for h in holders)
    assert [h['until'] for h in holders if h['until']]==['2022-04-06']
    print(json.dumps({'payload_files':len(expected),'reviewed_git_inputs':len(manifest['reviewed_git_inputs']),
      'external_files_checked':checked,'external_skipped':args.skip_external,'originals':7,'claims':27,'holder_observations':18,
      'integrity_only':True,'source_content_decisions':'separate recorded independent reading','runtime_or_country_qualification':False},sort_keys=True))

if __name__=='__main__':main()
