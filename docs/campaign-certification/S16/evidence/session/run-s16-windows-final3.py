"""Run clean-pinned final S16 Windows native lanes in dependency order."""
import pathlib,subprocess,sys
base=pathlib.Path(__file__).resolve().parent
pin=sys.argv[1]
fixture=base/'evidence/S16-fixture-final3'
for lane in ['fixture','binary','web','sim','integration']:
    cmd=[sys.executable,str(base/'run-s16-check.py'),pin,lane,'final3-'+lane]
    if lane=='fixture':cmd.append(str(fixture))
    print('START '+lane,flush=True)
    result=subprocess.run(cmd,cwd=base)
    print('FINISH '+lane+' '+str(result.returncode),flush=True)
    if result.returncode:sys.exit(result.returncode)
