import ctypes,json,sys,tempfile,unittest
from pathlib import Path
repo=Path(sys.argv[1]).resolve(strict=True)
parent=Path(r'C:\Users\ridge\AppData\Local\Temp\spheres-stability-alias-wt79ovk4').resolve(strict=True)
buffer=ctypes.create_unicode_buffer(32768)
assert ctypes.windll.kernel32.GetShortPathNameW(str(parent),buffer,len(buffer))>0
alias=Path(buffer.value)
assert alias.resolve()==parent and str(alias)!=str(parent)
tempfile.tempdir=str(alias)
sys.path.insert(0,str(repo/'tools/campaign'))
import test_stability_matrix
print(json.dumps({'parent':str(parent),'alias':str(alias),'resolved_equal':alias.resolve()==parent,'production_changed':False}),flush=True)
suite=unittest.defaultTestLoader.loadTestsFromName('RetainedVerificationTests.test_relocated_bundle_keeps_original_provenance_and_validates_locally',test_stability_matrix)
r=unittest.TextTestRunner(verbosity=2).run(suite)
sys.exit(not r.wasSuccessful())
