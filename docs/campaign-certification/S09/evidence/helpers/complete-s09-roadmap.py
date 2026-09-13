"""Update S09 documentation only after its collected qualification is complete."""
import gzip,hashlib,json,pathlib
b=pathlib.Path(__file__).resolve().parent;r=b/'integration';folder=r/'docs/campaign-certification/S09'
read=lambda p:json.loads(p.read_text(encoding='utf-8-sig'))
m=read(folder/'manifest.json');assert m['status']=='complete' and m['session']=='S09'
inventory=folder/'evidence/inventory.json'
assert hashlib.sha256(inventory.read_bytes()).hexdigest()==m['inventory']['sha256']
for item in read(inventory)['files']:
 chunks=[]
 for part in item['storage']:
  p=inventory.parent/part['file'];raw=p.read_bytes();assert len(raw)==part['bytes'] and hashlib.sha256(raw).hexdigest()==part['sha256'];chunks.append(raw)
 data=b''.join(chunks)
 if item['encoding']=='gzip':data=gzip.decompress(data)
 assert len(data)==item['bytes'] and hashlib.sha256(data).hexdigest()==item['sha256']
board=next(x for x in m['read_performance'] if x['id']=='board');previews=[x for x in m['read_performance'] if x['id']!='board'];worst=max(previews,key=lambda x:x['p95_ms'])
text=f'''# S09 — Research choices and readable design benefits

**S09 complete. Execution stopped before S10.** Qualified runtime: `{m['runtime_candidate']}`. [Open the separate review]({m['review']['url']}) and continue France to inspect the saved **S09 Atlas** design. Existing reviews and original campaigns remain intact.

The designer offers researched, compatible lower-cost and advanced starting configurations. Each shows role ratings, installation load, development funding, manufacturer costs, an indicative acquisition estimate and continuing upkeep. Component research explains the part, compatible vehicles and conditional effects; exploring it returns to the designer without silently replacing unsaved work. Explicit draft saving, named Save/Load and Continue retain the complete specification.

The roadmap's affordable/advanced recommendation item is implemented as lower-cost/advanced starting designs with visible funding conditions. A cheaper estimate is not an affordable finished-stock offer. Commissioning, purchasing and support still require their normal native review and funding; this session does not assert that every nation can buy every recommended design.

Qualification is bound in [manifest.json](manifest.json), with all selected records and reconstructed file hashes in [evidence/inventory.json](evidence/inventory.json).

- Full Windows and Linux native suites passed: 1,605 tests each, zero failures, 79 ignored; the ignored tests are not counted as passes.
- Full Node suites passed on both systems. Three original archive/lifecycle checks passed separately on each system, including the 28 active supplier fixtures.
- The real browser journey starts fresh France through ordinary menus. Suggestions recompile legally, research exploration issues no orders, and only one explicitly reviewed `equipment_save` command stores the draft. Cancelled loading is pure; named Save/Load and Continue preserve the exact complete native campaign.
- Native research-completion and frozen-revision regressions provide the no-free-equipment/no-retroactive-upgrade guarantees. The fresh browser campaign does not fabricate completed research or certified revisions.
- Thirteen desktop/mobile visual captures were inspected. The tank is visible after resizing, the cost/research sections are readable, and there is no horizontal page overflow. The earlier immediate blank frame was not reproduced after waiting for a settled frame; no renderer workaround or Reset action was needed.
- Original campaign and protected worktree preservation checks passed. The separate review's copied save was compared against the final browser archive using complete canonical bytes, including signed-zero preservation.

The read-performance protocol was declared before measurement: fresh paused France; sequential complete local HTTP/JSON responses; three discarded warmups and 21 retained samples for the board and each of eleven platform previews. Nearest-rank p95 limits are 300 ms for the board and 250 ms for previews; maxima are 750 ms and 500 ms.

| Read | Measured p95 | Measured maximum |
| --- | ---: | ---: |
| Complete equipment board | {board['p95_ms']:.2f} ms | {board['max_ms']:.2f} ms |
| Slowest platform p95: {worst['id']} | {worst['p95_ms']:.2f} ms | {worst['max_ms']:.2f} ms |

All twelve reads passed their unchanged limits in the completed journey. Every raw sample is retained. The first measurement had two isolated board stalls (313 and 324 ms) among otherwise 12–19 ms reads, failing board p95 at 313 ms. Added phase/GC observation did not reproduce the stalls; a subsequent full journey measured board p95 at 15.49 ms. Their cause remains unresolved, and no runtime performance improvement is claimed. The intervening test also exposed an unexported viewport helper; the S09 driver now contains the same assertions locally. Original waits, sample counts and limits remain unchanged. This is finite workload evidence, not a guarantee for maximum research, long campaigns or simulation-turn performance.

Startup investigation found an HTTP connection-pool admission race: a burst could queue more persistent connections than idle workers could start. The original pool starts only four of eight tasks in the deterministic regression; the local correction starts all eight. The exact vendored source is compiled by the ordinary workspace test. Both upstream licenses, per-file source hashes, the failing baseline, passing correction and actual Chrome connection logs are retained. Parsing, routing and gameplay rules are unchanged by the connection fix.

Earlier qualification attempts remain attributed to their own source, including the pre-fix startup timeouts, the retired `-modified` build stamp caused by Windows long-path Git detection, and the Linux source-preflight failure before native tests. Windows Git now recognizes the intact long paths. Linux compilation/tests remain native Linux; its final runner uses Windows Git for full source checks against the same NTFS checkout, avoiding incompatible metadata-cache scans. No timeout, gameplay response or campaign state was substituted to pass.

The existing S08 limitation for scanning directories with many large saves remains. This session awards no final campaign, worldwide roster or release certificate. S10 remains planned. Git transport is recorded separately after the containing completion commit exists.
'''
(folder/'README.md').write_text(text,encoding='utf-8')
path=r/'docs/planning/campaign-pathway.json';d=read(path);s=next(s for s in d['sessions'] if s['id']=='S09');assert s['status']=='in_progress'
s.update(status='complete',completed_date='2026-09-12',evidence='../campaign-certification/S09/README.md')
d['last_completed_session']='S09';d['next_session']='S10';assert d['execution']['stop_boundary']=='S09';d['execution']['status']='stopped'
path.write_text(json.dumps(d,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
path=r/'docs/CERTIFIED_CAMPAIGN_PATHWAY.md';text=path.read_text(encoding='utf-8')
text=text.replace('S01–S08 complete; S09 in progress.','S01–S09 complete; execution stopped before S10.')
text=text.replace('S01–S08 are complete; S09 is in progress; S10–S30 remain planned. Execution will stop after S09.','S01–S09 are complete; S10–S30 remain planned. Execution stopped after S09.')
start=text.index('#### S09 —');end=text.index('<a id="s10"></a>',start);block=text[start:end]
block=block.replace('**Status:** In progress','**Status:** Complete').replace('- [ ]','- [x]')
block+='Evidence and recommendation funding limits: [S09 qualification](campaign-certification/S09/README.md).\n\n'
text=text[:start]+block+text[end:]
text=text.replace('S09 is in progress; execution will stop before S10.','S09 is complete on its recorded runtime and evidence; execution stopped before S10.')
path.write_text(text,encoding='utf-8')
print('Verified every collected file and updated both S09 pathway markers; stopped before S10.')
