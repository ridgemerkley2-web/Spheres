import pathlib,shutil,json,hashlib,subprocess
base=pathlib.Path(__file__).resolve().parent;repo=base/'integration';pin='2c959cd7c4f61d09a77d5859b1cbee24aa509b10'
def read(p):return json.loads(pathlib.Path(p).read_text(encoding='utf-8-sig'))
def sha(p):
 with open(p,'rb') as f:return hashlib.file_digest(f,'sha256').hexdigest()
assert subprocess.check_output(['git','status','--porcelain'],cwd=repo,text=True).strip()==''
proofs=[]
for label,lanes in [('final1',['focused','manifest','records','budget-guard','browser','binary']),('linux1',['focused','manifest','records'])]:
 for lane in lanes:
  p=base/f'evidence/S22-prep-{label}-{lane}.json';proof=read(p)
  assert proof['passed'] and proof['revision']==pin and proof['clean_after']
  proofs.append((p,proof))
browser_dir=next((base/'evidence/S22-prep-final1-browser').glob('buffers-*'));browser=read(browser_dir/'result.json')
assert browser['passed'] and browser['source_revision']==pin and not browser['source_dirty']
assert len(browser['cases'])==12 and all(not row['shader_diagnostics'] and row['disposed']['live_buffers']==0 for row in browser['cases'])
native=read(base/'evidence/S22-prep-native-review/result.json');assert native['passed'] and native['revision']==pin
dest=repo/'docs/campaign-certification/S22/preparation';assert not dest.exists();evidence=dest/'evidence';evidence.mkdir(parents=True)
def copy(src,rel):
 target=evidence/rel;target.parent.mkdir(parents=True,exist_ok=True);shutil.copy2(src,target)
for p,proof in proofs:
 copy(p,'qualified/'+p.name);copy(p.with_suffix('.log'),'qualified/'+p.with_suffix('.log').name)
for directory,name in [(browser_dir,'browser-final'),(base/'evidence/S22-prep-native-review','native-review')]:
 for p in directory.iterdir():
  if p.is_file():copy(p,name+'/'+p.name)
for p in (base/'evidence/S22-preparation').glob('buffers-*/*'):
 if p.is_file():copy(p,'development/'+p.parent.name+'/'+p.name)
for p in (base/'evidence').glob('S22-prep-*-development*.log'):copy(p,'development/'+p.name)
for name in ['run-s22-prep-check.py','run-s22-prep-linux.sh','launch-s22-prep-review.py','s22-native-review.cjs','collect-s22-prep.py']:
 copy(base/name,'runners/'+name)
copy(base/'S22-prep-review-launch.json','review-launch.json');copy(base/'evidence/S22-prep-preservation.json','preservation.json')
# Preserve the Git metadata adapter used by the Linux runner.
adapter=subprocess.check_output(['wsl','-d','Ubuntu','--','cat','/home/ridge/.cache/spheres-s21-git/git'])
(evidence/'runners/linux-git-adapter.sh').write_bytes(adapter)
(dest/'.gitattributes').write_text('evidence/** -text -whitespace\n',encoding='utf8')
measure=read(repo/'docs/art/P0_MEASUREMENTS.json')
manifest={'packet':'CODEX-S22-PREP-01','status':'complete preparation; S22 qualification pending','source_revision':pin,
 'canonical_session_complete':False,'g4_complete':False,'cp1_certified':False,'budget_gate_passed':False,
 'checks':{'windows_focused':39,'linux_focused':39,'manifest_assets':33,'graded_configurations':len(measure['graded']),
 'over_budget_configurations':len(measure['over']),'browser_platforms':12,'actual_shadow_pass_required':True,'native_embedded_renderer_smoke':True},
 'limits':['No new full simulation suite: no native simulation changes. Existing S21 results remain at their own source revision.',
 'No FPS, driver VRAM, full city-cache, low-profile or early/mid/late campaign performance qualification.',
 'CPU accounting covers emitted typed arrays and backing allocations, not JS object overhead or all retained generator caches.',
 'WebGL accounting covers API buffer storage and submissions; textures, renderbuffers, CPU heap, driver overhead and rasterized pixels excluded.',
 'Initial development probes passed narrower storage assertions while falling back on a shader error. Final browser probe explicitly requires shader compilation and real offscreen draws.',
 'The first Linux launcher shell command had a PATH quoting error before any tests; corrected runner is retained.'],
 'runtime':read(base/'S22-prep-review-launch.json'),
 'files':[{'path':p.relative_to(dest).as_posix(),'bytes':p.stat().st_size,'sha256':sha(p)} for p in sorted(evidence.rglob('*')) if p.is_file()]}
(dest/'manifest.json').write_text(json.dumps(manifest,indent=2)+'\n',encoding='utf8')
(dest/'README.md').write_text('''# S22 preparation — art accounting and equipment shadows

**Bounded preparation complete**, qualified at `2c959cd7c4f61d09a77d5859b1cbee24aa509b10`.
**S22 remains planned; S20, full performance qualification, G4 and CP1 remain open.**
Claude still owns S19. No navigation or guidance files changed in this packet.
[Exact-source evidence and hashes](manifest.json).

The old audit stopped at the new CPU `materialClasses` array. The repaired tool
validates that layout, measures actual attribute lengths, counts CPU backing
buffers separately and sums payloads without assuming every attribute is uploaded.
Unexpected layouts still fail. The inventory now includes the fighter and all
three detail levels for nine ground vehicles and three CP1 aircraft. Source-weight
records use canonical LF text and reproduce on Windows and Linux.

Chrome exposed a real renderer defect: `packed` is reserved in the shader language.
The self-shadow receiver failed compilation, and the renderer silently used its
projected fallback. Renaming that local variable restores actual shadow-map
rendering. Geometry, game statistics, model budgets and campaign saves are unchanged.

| Standard baseline inspection | Before repair | After repair |
| --- | ---: | ---: |
| Tank triangles submitted during one orbit redraw | 137,746 | 68,874 |
| Fighter triangles submitted during one orbit redraw | 400,894 | 213,248 |
| Tank resident buffer payload | 10,744,248 bytes | 10,744,248 bytes |
| Fighter resident buffer payload | 21,648,384 bytes | 21,648,384 bytes |

The first shadow pass draws separately; later orbit redraws reuse it. These are
draw-submission and buffer-payload observations, **not FPS or total GPU memory**.
Shadow textures/renderbuffers are outside the buffer totals. Before observations
are development diagnostics, retained separately from the clean-commit final run.

## Checked

- 39 focused accounting, renderer and ownership regressions pass on both Windows
  and Linux. Generated manifest (33 assets) and measurement records reproduce on both.
- Chrome 153 / RTX 5070: all 12 platforms, LOD0 → LOD1 → LOD2 → LOD0, orbit,
  paint/selection, disposal and a fresh visit. Tank/fighter context loss and
  restoration rebuild the real shadow pass. A 390px fighter capture is retained.
- Final browser assertions require actual offscreen shadow submissions and no
  shader compiler diagnostics, so a working fallback cannot pass this check.
- The release executable builds; its served renderer bytes match the tested
  source. The actual equipment room renders self-shadows with no page errors,
  military orders or day advances in the native smoke check.
- Eight original save files and both protected worktrees pass preservation checks.

The pinned release is available locally at <http://127.0.0.1:7862/> with a separate
copy of the France campaign on 16 October 1992. Prior review runtimes remain intact.
The native smoke check opens the existing equipment entry function; it is not a
manual first-hour usability test.

## Open work

The original roadmap budget gate is **still failing**: 42 of 112 graded
configurations exceed its original ceilings. This includes later high-detail art;
limits were not widened and models were not reduced to hide the failures.
`bench_art.cjs --check-records` checks freshness only. `bench_art.cjs --check`
still exits 1, correctly identifying the overruns. All are listed in
[P0_BUDGETS.md](../../../art/P0_BUDGETS.md) and P0_MEASUREMENTS.json.

After S19 integration and S20, S22 must measure the agreed campaign workloads,
FPS/input response, cold loading, tick throughput, memory, city/inspection caches
and low-detail profile at early/mid/late dates. This repair supplies trustworthy
tools and fixes a reproduced defect; it does not replace that qualification.
''',encoding='utf8')
handoff=repo/'docs/planning/ai-handoffs/CODEX-S22-PREP-01.md'
s=handoff.read_text(encoding='utf8').replace('State: implementation prepared; exact-source validation pending.','State: **complete bounded preparation**, qualified at `2c959cd`.\n[Validation, remaining work and evidence](../../campaign-certification/S22/preparation/README.md).')
handoff.write_text(s,encoding='utf8')
board=repo/'docs/AI_WORKSTREAMS.md'
with board.open('a',encoding='utf8') as f:f.write('\nCodex completed [CODEX-S22-PREP-01](planning/ai-handoffs/CODEX-S22-PREP-01.md): repaired art accounting, restored compiled equipment self-shadows, and validated the isolated renderer. This is independent preparation; S22 remains planned after S20. The original art budget gate still reports 42 overruns.\n')
work=repo/'docs/planning/ai-workstreams.json';data=read(work)
next(w for w in data['workstreams'] if w['id']=='qualification')['next']='CODEX-S22-PREP-01 complete at 2c959cd: repaired art accounting and equipment self-shadows. S22 remains planned after S20; 42 art budget overruns and full campaign performance qualification remain open. Evidence: docs/campaign-certification/S22/preparation/README.md.'
work.write_text(json.dumps(data,indent=2,ensure_ascii=False)+'\n',encoding='utf8')
print(json.dumps({'files':len(manifest['files']),'bytes':sum(f['bytes'] for f in manifest['files']),'path':str(dest)},indent=2))
