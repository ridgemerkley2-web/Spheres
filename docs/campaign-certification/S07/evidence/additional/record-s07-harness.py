import pathlib,shutil,json,hashlib,sys
base=pathlib.Path(__file__).resolve().parent;repo=base/'integration';dest=base/'evidence/S07-harness-sources'
dest.mkdir(exist_ok=True)
sha=lambda p:hashlib.file_digest(p.open('rb'),'sha256').hexdigest()
for name,source in [('ci-construction-original.cjs',repo/'tools/ui/ci-construction.cjs'),('ci-construction-renewal-candidate.cjs',repo/'artifacts/s07-renewal-harness/ci-construction.cjs'),('ci-browser-candidate.cjs',repo/'artifacts/s07-startup-harness/ci-browser.cjs'),('candidate-wrapper.cjs',repo/'artifacts/s07-startup-harness/run.cjs')]:
 target=dest/name
 if target.exists():assert sha(target)==sha(source)
 else:shutil.copy2(source,target)
if len(sys.argv)==1:sys.exit(0)
tonga=pathlib.Path(sys.argv[1]).resolve();general=pathlib.Path(sys.argv[2]).resolve();france=repo/'artifacts/browser-construction-ci/france-xUfR8c/result.json'
rows={}
for n,r,h in [('France',france,'ci-construction-original.cjs'),('Tonga',tonga,'ci-construction-renewal-candidate.cjs'),('general',general,'ci-browser-candidate.cjs')]:
 value=json.loads(r.read_text(encoding='utf-8-sig'));assert value.get('passed',value.get('ok',False))
 rows[n]=dict(result=str(r.resolve()),result_sha256=sha(r),harness=str(dest/h),harness_sha256=sha(dest/h))
value=dict(runtime_revision='041007fbfda48cc027d0c24a1189705a1dcaa89b',runtime_unchanged=True,correction_summary='France passed the committed gameplay harness unchanged. Tonga and general regression use bounded /api/build readiness probes whose bodies are fully consumed. The selected Tonga journey additionally checks annual funding once per observed year and reapplies the same daily budget through the visible Apply control when renewal is required. It asserts unchanged date, project progress and paid work, then continues actual daily advances. The original 800-day bound and accounting, save and outcome assertions remain. An earlier Tonga run omitted renewal and failed at that bound; its full evidence is retained. Candidates use Module._compile at original filenames; general keeps the supported installed Chrome channel override.',runtime_limitation='The existing synchronous HTTP response loop can block behind an unread response. The probe establishes this mechanism for the Tonga-shaped startup failure. The earlier France navigation timeout remains of unproven cause; the runtime was not changed to add slow-reader isolation.',runs=rows,wrapper=dict(path=str(dest/'candidate-wrapper.cjs'),sha256=sha(dest/'candidate-wrapper.cjs')))
(base/'evidence/S07-harness-correction.json').write_text(json.dumps(value,indent=2)+'\n')
print(json.dumps(value,indent=2))
