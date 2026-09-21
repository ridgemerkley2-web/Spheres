import pathlib,subprocess,sys
base=pathlib.Path(__file__).resolve().parent
pin=sys.argv[1]
fixture=base/'evidence/S17-fixture-final5'
for lane in ['fixture','binary','browser','web','node']:
    cmd=[sys.executable,str(base/'run-s17-check.py'),pin,lane,'final5-'+lane]
    if lane in ['fixture','browser']:cmd.append(str(fixture))
    print('START '+lane,flush=True)
    result=subprocess.run(cmd,cwd=base)
    print('FINISH '+lane+' '+str(result.returncode),flush=True)
    if result.returncode:sys.exit(result.returncode)


