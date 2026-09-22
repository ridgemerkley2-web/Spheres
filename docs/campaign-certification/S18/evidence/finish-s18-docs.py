import json,pathlib
base=pathlib.Path(__file__).resolve().parent;repo=base/'integration';dest=repo/'docs/campaign-certification/S18';m=json.loads((dest/'manifest.json').read_text(encoding='utf8'));assert m['status']=='complete'
pin=m['candidate_revision'];w=m['qualification']['windows'];linux={c['name']:c for c in m['qualification']['linux']['checks']}
assert w['web']['totals']==linux['web']['totals'];assert w['node']['totals']['pass']==linux['node']['totals']['passed']
p=dest/'README.md';s=p.read_text(encoding='utf8');a=s.index('Status:');b=s.index('\n\nAir command',a);s=s[:a]+f'Status: **complete** on `{pin}`. [Exact qualification and retained evidence](manifest.json). G4 and CP1 remain open.'+s[b:]
a=s.index('Planned qualification:');b=s.index('\n\nOriginal artwork',a)
s=s[:a]+f'''Qualification passed on Windows and Linux: **{w['web']['totals']['passed']} native web tests** ({w['web']['totals']['ignored']} ignored) and **{w['node']['totals']['pass']} Node tests** ({w['node']['totals']['skipped']} skipped) per platform. Asset regeneration and the native fixture export also passed.

Runtime/native/browser/art evidence is pinned to `{pin}`. The final Node runs use
`{m['test_revision']}`; its only change corrects the existing inlet-depth test's
sampling coordinate to the new duct position. The manifest verifies that exact
one-line test-only difference. Failed initial runs remain in the evidence archive.

The ordinary browser route checked all four campaign pages at 1440px and 390px,
downloaded actual owned specifications, retained the local draft, then reviewed two
staff commands and advanced six ordinary days. **Ten exact native-world and ten
history-envelope comparisons** passed through inspection, commands, Save, Load and
Continue, with zero browser errors. No world fields were ignored; only the separately
validated save timestamp can differ in an envelope. Authored starting forces are
disclosed; this is not an unassisted long-campaign or human-usability claim.

Thirteen final art captures plus the live-page/campaign captures were inspected.
The art browser exercised all three families, eight selectable slots, cockpit,
engine and inlet cameras, LOD switching and real GLB downloads. Original saves and
the two protected worktree heads are unchanged. No simulation code or dependency
changes were made; S17 retains its separately pinned simulation/integration evidence.
Follow-up polish for S20: normalize signed-zero display values and replace raw
province/ammunition identifiers in inherited mission report prose with friendly names.
The restore toast is transient and is visible in the immediately captured page shots.

[Open the separately copied S18 review campaign]({m['review']['url']}). The older S17
review remains available. S19 is assigned to Claude in the shared workboard; its
handoff is now dependency-ready. Codex's S20 shared-navigation work waits for that handoff.''' +s[b:];p.write_text(s,encoding='utf8')
p=repo/'docs/planning/campaign-pathway.json';d=json.loads(p.read_text(encoding='utf8'));session=next(x for x in d['sessions'] if x['id']=='S18');assert session['status']=='in_progress';session.update(status='complete',completed_date='2026-09-21',evidence='../campaign-certification/S18/README.md',closure={'evidence':'docs/campaign-certification/S18/manifest.json','candidate_revision':pin,'completed_utc':m['completed_utc']});d.update(last_completed_session='S18',next_session='S19');d['execution'].update(status='stopped',next_session_requires_instruction=True);p.write_text(json.dumps(d,ensure_ascii=False,indent=2)+'\n',encoding='utf8')
p=repo/'docs/CERTIFIED_CAMPAIGN_PATHWAY.md';s=p.read_text(encoding='utf8').replace('S01–S17 complete; S18 in progress; S19–S30 planned.','S01–S18 complete; S19–S30 planned.');a=s.index('#### S18 —');b=s.index('<a id="s19">',a);section=s[a:b].replace('- [ ]','- [x]').replace('**Status:** Planned','**Status:** Complete').replace('**Status:** In progress','**Status:** Complete');section+='**Evidence:** [S18 native pages, aircraft art and exact browser qualification](campaign-certification/S18/README.md). S19 is assigned to Claude through the AI workboard. G4 and CP1 remain open.\n\n';s=s[:a]+section+s[b:];p.write_text(s,encoding='utf8')
p=repo/'README.md';s=p.read_text(encoding='utf8').replace('S01–S17 are complete; S18 is in development.','S01–S18 are complete; S19 is next and assigned to Claude.');p.write_text(s,encoding='utf8')
p=repo/'docs/AI_WORKSTREAMS.md';s=p.read_text(encoding='utf8').replace('S01–S17 are complete;\nCodex is implementing S18.','S01–S18 are complete;\nClaude’s S19 handoff is dependency-ready.').replace('Finish S18’s live flight pages and aircraft. Then navigation and campaign goals.','S18 complete. Navigation follows the S19 handoff; campaign goals remain planned.').replace('Design/review now; implementation starts after S18 is merged.','Dependency-ready after S18; claim the S19 handoff when instructed.');p.write_text(s,encoding='utf8')
p=repo/'docs/planning/ai-handoffs/CLAUDE-S19-01.md';s=p.read_text(encoding='utf8').replace('**waiting for S18 integration**','**dependency-ready after S18; not yet claimed**');p.write_text(s,encoding='utf8')
p=repo/'docs/planning/ai-workstreams.json';d=json.loads(p.read_text(encoding='utf8'))
for stream in d['workstreams']:
 if stream['id']=='player-journey':stream['next']='S18 complete. Integrate Claude S19 before shared-shell changes for S20; S21 remains planned.'
 if stream['id']=='guidance':stream['next']='CLAUDE-S19-01 is dependency-ready after S18; claim one bounded packet when instructed.'
p.write_text(json.dumps(d,indent=2)+'\n',encoding='utf8')
print('Closed S18; S19 dependency-ready for Claude. G4 and CP1 remain open.')
