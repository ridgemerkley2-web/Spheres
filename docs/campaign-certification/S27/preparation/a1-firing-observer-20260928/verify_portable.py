"""Verify the portable A1 packet without native code or original source paths.
Optional --restore-run NEW_DIRECTORY recreates the five original run files only.
"""
from pathlib import Path, PurePosixPath
import argparse, collections, gzip, hashlib, json, shutil

def fingerprint(path, compressed=False):
    digest=hashlib.sha256(); size=0
    with (gzip.open(path,'rb') if compressed else path.open('rb')) as stream:
        while data:=stream.read(1024*1024): digest.update(data); size+=len(data)
    return size,digest.hexdigest()

def verify(root, restore=None):
    root=root.resolve(); manifest=json.loads((root/'manifest.json').read_text(encoding='utf-8'))
    assert manifest['candidate_revision']=='6818e4f0d94b01c86d7a9acc4252260947d13504'
    assert manifest['portable_original_diagnostic_data_complete'] is True
    assert manifest['qualification'] is False and manifest['calibration_pass_claimed'] is False
    declared=set(); decoded={}
    for item in manifest['files']:
        relative=PurePosixPath(item['path'])
        assert not relative.is_absolute() and '..' not in relative.parts and '\\' not in item['path']
        assert item['path'] not in declared; declared.add(item['path'])
        path=root.joinpath(*relative.parts)
        assert path.is_file() and not path.is_symlink() and path.resolve().is_relative_to(root)
        assert fingerprint(path)==(item['bytes'],item['sha256']),item['path']
        if item['encoding']=='gzip':
            observed=fingerprint(path,True)
            assert observed==(item['original_bytes'],item['original_sha256']),item['path']
            decoded[item['path']]=observed
            with path.open('rb') as f: header=f.read(10)
            assert header[:3]==b'\x1f\x8b\x08' and header[3]==0 and header[4:8]==b'\x00'*4
        else: assert item['encoding']=='identity'
    actual={p.relative_to(root).as_posix() for p in root.rglob('*') if p.is_file() and p != root/'manifest.json'}
    assert actual==declared,(actual-declared,declared-actual)
    assert set(decoded)=={'original-run/observations.jsonl.gz','original-run/control-final.json.gz','original-run/observed-final.json.gz'}
    assert decoded['original-run/control-final.json.gz']==decoded['original-run/observed-final.json.gz']
    with gzip.open(root/'original-run/control-final.json.gz','rb') as a, gzip.open(root/'original-run/observed-final.json.gz','rb') as b:
        while True:
            left=a.read(1024*1024); right=b.read(1024*1024)
            assert left==right
            if not left: break
    result=json.loads((root/'original-run/result.json').read_text(encoding='utf-8'))
    assert result['passed'] is True and result['qualification'] is False and result['a1_pass_claimed'] is False
    assert result['months_compared']==252 and len(result['comparisons'])==252 and all(r['equal'] for r in result['comparisons'])
    stages=collections.Counter(); firings=[]
    with gzip.open(root/'original-run/observations.jsonl.gz','rt',encoding='utf-8') as stream:
        for line in stream:
            row=json.loads(line); stages[row['stage']]+=1
            if row['stage']=='trigger_firing': firings.append(row)
    assert dict(stages)==result['stage_counts'] and sum(stages.values())==146525
    assert firings==result['firing_cases'] and len(firings)==10
    execution=json.loads((root/'execution/execution.json').read_text(encoding='utf-8'))
    assert execution['source_revision']==manifest['candidate_revision'] and execution['exit_code']==0
    assert execution['binary_sha256']==execution['binary_sha256_after']==manifest['binary_external_reference']['sha256']
    old=json.loads((root/'analysis/manifest.json').read_text(encoding='utf-8'))
    for item in old['artifacts']:
        assert fingerprint(root/'analysis'/item['path'])==(item['bytes'],item['sha256'])
    if restore is not None:
        assert restore.is_absolute()
        restore=restore.resolve()
        assert not restore.exists() and not restore.is_relative_to(root) and not root.is_relative_to(restore)
        restore.mkdir(parents=False,exist_ok=False)
        for name in ['plan.json','result.json','observations.jsonl','control-final.json','observed-final.json']:
            compressed=name not in ['plan.json','result.json']; src=root/'original-run'/(name+'.gz' if compressed else name)
            with (gzip.open(src,'rb') if compressed else src.open('rb')) as f,(restore/name).open('xb') as target:
                shutil.copyfileobj(f,target,1024*1024)
            expected=decoded['original-run/'+name+'.gz'] if compressed else fingerprint(src)
            assert fingerprint(restore/name)==expected
    return {'verified_files':len(declared),'rows':sum(stages.values()),'firings':len(firings),'decoded_archives':3,'final_pair_equal':True,'restored_run':str(restore) if restore else None,'native_execution_performed':False,'qualification':False}

if __name__=='__main__':
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--root',type=Path,default=Path(__file__).resolve().parent)
    parser.add_argument('--restore-run',type=Path)
    args=parser.parse_args(); print(json.dumps(verify(args.root,args.restore_run),indent=2))
