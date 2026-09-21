"""Close only the qualified S17 session; leave S18, G4 and CP1 open."""
import json,pathlib
base=pathlib.Path(__file__).resolve().parent;repo=base/'integration';dest=repo/'docs/campaign-certification/S17'
m=json.loads((dest/'manifest.json').read_text(encoding='utf8'));assert m['status']=='complete'
pin=m['candidate_revision']; completed=m['completed_utc'];date=completed[:10]
windows=m['qualification']['windows'];linux={c['name']:c for c in m['qualification']['linux_simulation']['checks']}
linux.update({c['name']:c for c in m['qualification']['linux_node']['checks']})
linux.update({c['name']:c for c in m['qualification']['linux']['checks']})
for lane in ['web','sim','integration']:
    assert windows[lane]['totals']==linux[lane]['totals']
assert windows['node']['totals']['pass']==linux['node']['totals']['passed']
rows=[]
for label,lane in [('Native web','web'),('Native simulation','sim'),('13 integration targets','integration'),('Node interface','node')]:
    totals=windows[lane]['totals'];passed=totals['pass'] if lane=='node' else totals['passed'];ignored=totals['skipped'] if lane=='node' else totals['ignored']
    rows.append(f'| {label} | {passed} | {ignored} |')
readme=(dest/'README.md').read_text(encoding='utf8')
readme=readme.replace('Status: implementation and qualification in progress. Authorized by “next\'” on 21 September 2026. S18 is outside this session. G4 and CP1 remain open.',f'Status: **complete** on runtime `{pin}`. Authorized by “next\'” on 21 September 2026. Execution stops after S17; S18 is next. G4 and CP1 remain open.')
assert 'Qualification results, exact source revision and browser comparisons will be recorded after the final checks.' in readme
readme=readme.replace('Qualification results, exact source revision and browser comparisons will be recorded after the final checks.',f'''## Qualification

Both **Windows and Linux** passed the same complete selected suites on the recorded clean sources. Simulation/integration qualify `{m['qualification']['source_continuity']['simulation_revision']}`. Node qualifies `{m['qualification']['source_continuity']['node_revision']}`, with identical JavaScript/CSS at the final runtime. Only report CSS, browser screenshot positioning and two pure currency-format calls differ from the simulation candidate. Native web, fixture and browser checks qualify the final runtime above. The manifest records and verifies these exact source comparisons:

| Suite (each platform) | Passed | Ignored / skipped |
|---|---:|---:|
{chr(10).join(rows)}

The separate native fixture export also passed. Ignored and skipped cases are counted separately and are not passes. Direct native coverage includes France's paid supplier development, tooling, finite production, purchase and delivery; a one-aircraft Malta import without research or factory grants; ground ammunition purchasing and delivery; bounded research and stock rotation; political cost and future-only maintenance reallocations; cash, upkeep and access refusals; and deterministic saved review cadence. Authored opposing forces also exercise coordinated real attack/defense and finite stores.

The browser used **two ordinary reviewed settings commands and six one-day advances**, with **{m['qualification']['browser']['world_comparisons']} full native-world comparisons** and **{m['qualification']['browser']['envelope_comparisons']} historical-envelope comparisons**. No world fields were ignored; only the separately validated save timestamp may differ in an envelope. Enable, pause, Save, Load and Continue passed with no browser errors. Desktop 1440px and narrow 390px captures were inspected directly; country reports and controls remain readable using vertical scrolling. This is functional visual review, not independent human usability certification or S18 aircraft art approval.

The scenario retains the disclosed S16 France/Italy forces, bases, ammunition and finances and explicitly enables economic competition before the user-reviewed staff setting. Staff move Italy's future Defense funding toward maintenance, wait for paid service, then autonomously order Support army missions against actual campaign contacts. The first new mission consumes approximately **2.63 finished bomb stores**. A previously queued second mission completes after pausing; saved staff decisions remain unchanged. No new aircraft, money, research, ammunition or usable base capacity is granted by enabling staff. This bounded six-day scenario does not claim an unassisted campaign or historically exhaustive opening forces.

The [manifest](manifest.json) retains exact source/binary hashes, reports, complete qualifying logs, failed-attempt context, screenshots, runners and raw native/browser campaign checkpoints. Its file inventory records original and stored SHA-256 hashes; `.gz` entries decompress to the exact original bytes. Duplicate canonical worlds and compiled binaries are identified rather than repackaged. The baseline's eight protected files and two worktree HEADs remain unchanged.

[Open the separate S17 review campaign]({m['review']['url']}). Its saved **active** checkpoint matches the native/browser comparison; the existing S16 review remains available separately. **Next: S18 — campaign flight screens and finished inspection aircraft.** G4, later campaign/content/performance qualification and CP1 remain open.''')
(dest/'README.md').write_text(readme,encoding='utf8')
roadmap=repo/'docs/planning/campaign-pathway.json';data=json.loads(roadmap.read_text(encoding='utf8'))
s=next(s for s in data['sessions'] if s['id']=='S17');assert s['status']=='in_progress'
s.update(status='complete',evidence='../campaign-certification/S17/README.md',completed_date=date,
    closure={'evidence':'docs/campaign-certification/S17/manifest.json','candidate_revision':pin,'completed_utc':completed})
data['last_completed_session']='S17';data['next_session']='S18';data['date']=date
data['execution'].update(status='stopped',next_session_requires_instruction=True)
roadmap.write_text(json.dumps(data,indent=2,ensure_ascii=False)+'\n',encoding='utf8')
path=repo/'docs/CERTIFIED_CAMPAIGN_PATHWAY.md';text=path.read_text(encoding='utf8')
text=text.replace('S01–S16 complete; S17–S30 planned.','S01–S17 complete; S18–S30 planned.').replace('S01–S16 are complete; S17–S30 remain planned.','S01–S17 are complete; S18–S30 remain planned.')
start=text.index('#### S17 —');end=text.index('<a id="s18">',start)
section=text[start:end].replace('**Status:** In progress','**Status:** Complete').replace('- [ ]','- [x]')
section+='**Evidence:** [S17 qualification and policy boundaries](campaign-certification/S17/README.md). Both native platforms, exact browser checkpoints and desktop/narrow review passed. G4 and CP1 remain open.\n\n'
text=text[:start]+section+text[end:]
text+='\nOn 21 September 2026, “next\'” authorized S17. S17 is complete on its [recorded runtime and evidence](campaign-certification/S17/README.md). Autonomous staff use ordinary paid acquisition, supplier development, support, basing and supported air commands. Execution stops after S17, with S18 awaiting a new instruction. G4 and CP1 remain open.\n'
path.write_text(text,encoding='utf8')
print('S17 documentation complete; S18 remains planned.')
