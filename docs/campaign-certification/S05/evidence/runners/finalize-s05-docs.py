from pathlib import Path
import json,datetime,subprocess,shutil,hashlib
base=Path(__file__).resolve().parent;repo=base/'integration';ev=base/'evidence';staging=base/'s05-staging/closeout'
read=lambda p:p.read_text(encoding='utf-8-sig')
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
candidate='db9d17c8d726aa102aa143ceb3599009c558ffee'
record=json.loads(read(staging/'final-record.json'))
assert record['runtime_candidate']==candidate
for name in ['S05-lifetime-confirmation-1','S05-lifetime-confirmation-2']:
    p=json.loads(read(ev/name/'profile.json'));r=json.loads(read(ev/name/'runner-result.json'))
    assert r['passed'] and r['candidate_revision']==candidate and r['memory']['max_sampled_private_bytes']<=1024**3
    assert len(p['results'])==6
    for x in p['results']:
        assert x['simulation_and_history_recording']['p95_ms']<=300 and x['whole_server_turn']['p95_ms']<=400 and x['whole_server_turn']['max_ms']<=750
record.update(completed_date='2026-09-10',decisions={'S04':'complete','S05':'complete','G1':'earned'},qualification_gates_passed=True,
 gate_reason='The pinned integrated build passes both native/Node platforms, original-save and transaction qualification, actual desktop/narrow browser journeys, map review and both predefined performance confirmations. This earns the bounded unified-playset gate locally; full campaign and release certification remain later work.',
 note='Final gate decision follows exact db9 execution, retained intermediate failures, the fixed two-run timing protocol and two passing confirmations. No later session is authorized by this decision.')
v=record['replacements']
v['FINAL_PERFORMANCE_STATUS']='both predefined full six-case confirmations passed every unchanged latency/memory bar; the initial optimized timing miss is retained'
v['FINAL_PERFORMANCE_EVIDENCE']='**PASS for both predefined confirmations**, each six copied workloads × 31 measured days with connected economy, company operations and warfare adopted. Simulation p95 ≤300 ms, whole-turn p95 ≤400 ms, whole maximum ≤750 ms and sampled private memory ≤1 GiB remain unchanged. Busy-year-30 p95 was **291.268/397.802 ms** (simulation/whole) in [confirmation 1](evidence/S05-lifetime-confirmation-1/profile.json) and **261.161/359.921 ms** in [confirmation 2](evidence/S05-lifetime-confirmation-2/profile.json). Sampled private maxima were 445,026,304 and 423,804,928 bytes. The [fixed protocol](evidence/S05-performance-confirmation-protocol.json) required both runs to pass, with no further retries. The earlier optimized run missed 300/400 ms at **303.450/413.267 ms**, and the pre-optimization run had larger failures; both remain recorded. Runner `passed` validates profile execution/structure; the stricter separate qualification also checks every numerical bar. The near-limit first confirmation leaves little headroom: timing variation is observed without an inferred cause, and S22 owns broader performance qualification.'
v['FINAL_PERFORMANCE_EVIDENCE_S04']=v['FINAL_PERFORMANCE_EVIDENCE'].replace('(evidence/','(../S05/evidence/')
v['S04_DECISION']='complete for the recorded operational warfare and coupled-map integration scope'
v['S05_DECISION']='complete for the recorded save, startup, command and browser qualification scope'
v['G1_DECISION']='earned locally for the pinned unified playset; full 1990–2035 campaign certification remains unearned'
v['FINAL_GIT_EVIDENCE']='Runtime `db9d17c8d726aa102aa143ceb3599009c558ffee` was clean before and after the final suites. The closeout commit adds only documentation/evidence. Original active and master sources, eight protected saves and original qualification inputs remain unchanged. GitHub publication is separate: recorded preflight push attempts lacked authentication; the final push outcome is reported in the task. No hosted GitHub Actions result is claimed.'
(staging/'final-record.json').write_text(json.dumps(record,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
subprocess.run(['node',str(staging/'prepare-closeout.cjs'),str(staging/'final-record.json'),str(repo)],cwd=repo,check=True)
for relative in ['docs/campaign-certification/S04/README.md','docs/campaign-certification/S05/README.md','docs/planning/campaign-pathway.json','docs/CERTIFIED_CAMPAIGN_PATHWAY.md']:
    shutil.copyfile(staging/'ready'/relative,repo/relative)
dest=repo/'docs/campaign-certification/S05/evidence'
for source,name in [(staging/'final-record.json','closeout-decision.json'),(staging/'prepare-closeout.cjs','runners/prepare-closeout.cjs'),(base/'finalize-s05-docs.py','runners/finalize-s05-docs.py')]:
    (dest/name).parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(source,dest/name)
comparison=json.loads(read(ev/'S05-deployment-outcome-optimized/result.json'))
extra='\n## Preservation and routing equivalence\n\nThe [final preservation audit](evidence/S05-preservation-runtime-final.json) confirms all eight protected saves, all 28 original active fixture files, the genuine master archive, four producer binaries and two original source worktrees remain unchanged. Original active HEAD is `038fe0b1a3edb1772586ec0b5a7451629b7297c4`; its runtime/data baseline remains `5f7f355502f17bd6bd8f0383a2d14f0024fa7884`. The original master checkout remains exact `485c223f60d5ff6e46f6ae17164bf1ee3a8764d9`.\n\nThe [pre/post routing comparison](evidence/S05-deployment-outcome-optimized/result.json) loads the same genuine master archive on pre-optimization93 and final db9, advances 31 normal days, and reloads its native save at day15. All five checkpoint archives match exactly, including world, history, dispatch log and RNG. Only top-level wall-clock `saved_unix` is omitted; numeric tokens retain exact u64/floating-point values. The final canonical archive SHA-256 is `'+comparison['checkpoints'][-1]['canonical_sha256']+'`. Full output archives stay local; their hashes, requests and runner are packaged with [original input extraction instructions](evidence/S05-fixture-archive/README.md). This is a real technical war workload, not a claim of a certified USA campaign.\n'
p=repo/'docs/campaign-certification/S05/README.md';p.write_text(read(p)+extra,encoding='utf-8')
for session in ['S04','S05']:
    out=repo/f'docs/campaign-certification/{session}';p=out/'manifest.json';d=json.loads(read(p))
    d.update(status='complete',gate='G1 unified playset earned locally' if session=='S05' else 'S04 scope complete',native_linux={'passed':1480,'failed':0,'ignored':75},node_linux={'passed':1472,'failed':0,'skipped':1},qualification_limits=['First optimized timing trial missed the oldest busy p95 bars; both predefined confirmations passed unchanged limits, with narrow headroom retained for S22.','75 native ignored cases are not default-suite passes; the three original archive checks ran separately on each OS.','Local WSL native/Node evidence and Windows Chrome browser evidence; no Linux browser or hosted CI claim.','No full 1990–2035, all-country/successor, human-playtest or packaged-release certificate.'],stop_boundary='S05')
    d['files']=[{'path':x.relative_to(out).as_posix(),'sha256':sha(x),'bytes':x.stat().st_size} for x in sorted((out/'evidence').rglob('*')) if x.is_file()]
    p.write_text(json.dumps(d,indent=2)+'\n',encoding='utf-8')
print('S04/S05 reports and roadmap finalized; S06 remains planned.')
