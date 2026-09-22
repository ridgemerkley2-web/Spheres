import pathlib,subprocess,sys
base=pathlib.Path(__file__).resolve().parent
pin=sys.argv[1];fixture=base/'evidence/S18-fixture-final1'
for lane in ['node','assets','art','web','fixture','binary','browser']:
 cmd=[sys.executable,str(base/'run-s18-check.py'),pin,lane,'final1-'+lane]
 if lane in ['fixture','browser']:cmd.append(str(fixture))
 print('START '+lane,flush=True);r=subprocess.run(cmd,cwd=base);print('FINISH '+lane+' '+str(r.returncode),flush=True)
 if r.returncode:sys.exit(r.returncode)
