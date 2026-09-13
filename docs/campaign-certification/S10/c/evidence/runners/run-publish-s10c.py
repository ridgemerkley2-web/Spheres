import pathlib, subprocess, sys
base=pathlib.Path(__file__).resolve().parent
args=[sys.executable,'-X','utf8','publish-s10c.py',
 '--runtime-pin','c6c350fbf5b22f83b47f4f47997f5a1d0200ab2c',
 '--web-proof','evidence/S10-c-web-3.json',
 '--node-proof','evidence/S10-c-node-3.json',
 '--binary-proof','evidence/S10-c-binary-3.json',
 '--linux-proof','evidence/S10c-linux-final-2/runner-result.json',
 '--matrix-result','evidence/S10c-matrix-final-2/matrix-4aP4Fd/result.json',
 '--matrix-wrapper','evidence/S10c-matrix-driver-final-2.json',
 '--browser-result','evidence/S10-browser-c-browser-2/france-uhJF99/result.json',
 '--browser-wrapper','evidence/S10-c-browser-2.json']
failures={
 'initial-node':'evidence/S10-c-node-1.json',
 'initial-linux-superseded':'evidence/S10c-linux-final-1',
 'initial-matrix-wrapper':'evidence/S10c-matrix-driver-final-1.json',
 'initial-matrix-results':'evidence/S10c-matrix-final-1/matrix-RuSiS6/result.json',
 'initial-france-matrix':'evidence/S10c-matrix-final-1/matrix-RuSiS6/france-Gk38XM',
 'initial-policy-wrapper':'evidence/S10-c-browser-1.json',
 'superseded-web':'evidence/S10-c-web-2.json',
 'superseded-node':'evidence/S10-c-node-2.json',
}
originals=list((base/'evidence/S10-browser-c-browser-1').glob('france-*'))
assert len(originals)==1
failures['initial-policy']=str(originals[0])
for label,path in failures.items():args+=['--failed-attempt',label+'='+path]
args+=['--additional-evidence','portrait-review=evidence/S10c-tupou-portrait-checks.json']
subprocess.run(args,cwd=base,check=True)
