"""Read-only owner/status query. Completion comes only from the canonical pathway."""
import argparse, json, pathlib, sys

root = pathlib.Path(__file__).resolve().parents[2]
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--owner', choices=['Codex', 'Claude', 'Human'])
parser.add_argument('--session')
parser.add_argument('--check', action='store_true')
args = parser.parse_args()
board = json.loads((root/'docs/planning/ai-workstreams.json').read_text(encoding='utf-8'))
plan = json.loads((root/board['status_source']).read_text(encoding='utf-8'))
sessions = {s['id']: s for s in plan['sessions']}
owners = {}
for stream in board['workstreams']:
    for sid in stream['sessions']:
        if sid in owners or sid not in sessions:
            sys.exit('Duplicate or unknown assignment: '+sid)
        owners[sid] = stream
    if stream.get('handoff') and not (root/stream['handoff']).is_file():
        sys.exit('Missing handoff: '+stream['handoff'])
if set(owners) != set(sessions):
    sys.exit('Unassigned markers: '+', '.join(sorted(set(sessions)-set(owners))))
for sid, session in sessions.items():
    dependencies = session['depends']+board['coordination_dependencies'].get(sid, [])
    if any(d not in sessions for d in dependencies):
        sys.exit('Unknown dependency on '+sid)
if args.check:
    print(f'PASS: {len(sessions)} canonical markers, each assigned once; handoffs and dependencies exist. No status is duplicated.')
    sys.exit(0)
if args.session and args.session not in sessions:
    sys.exit('Unknown session: '+args.session)
print('Integration branch: '+board['integration_branch'])
print('Most recently completed: '+plan['last_completed_session']+'; next canonical session '+plan['next_session'])
for sid, session in sessions.items():
    stream = owners[sid]
    if args.owner and stream['owner'] != args.owner: continue
    if args.session and sid != args.session: continue
    if not args.session and session['status'] == 'complete': continue
    unmet = [d for d in dict.fromkeys(session['depends']+board['coordination_dependencies'].get(sid, [])) if sessions[d]['status'] != 'complete']
    print(f"\n{sid} | {stream['owner']} | {session['status']} | {session['title']}")
    print('  Waiting for: '+(', '.join(unmet) if unmet else 'no incomplete roadmap dependency; use the bounded handoff'))
    print('  Output: '+session['output'])
    print('  Next: '+stream['next'])
    if stream.get('handoff'): print('  Handoff: '+stream['handoff'])
    if session.get('evidence'): print('  Evidence: '+session['evidence'])
    if args.session:
        for criterion in session['accept']: print('  Acceptance: '+criterion)
