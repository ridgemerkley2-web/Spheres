"""Read-only owner/status query. Completion comes only from the canonical pathway."""
import argparse, json, pathlib, sys

root = pathlib.Path(__file__).resolve().parents[2]
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--owner', choices=['Codex', 'Claude', 'Human'])
parser.add_argument('--session')
parser.add_argument('--tasks', action='store_true', help='Show bounded tasks in priority order rather than canonical sessions')
parser.add_argument('--task', help='Read one exact bounded task ID')
parser.add_argument('--check', action='store_true')
args = parser.parse_args()
board = json.loads((root/'docs/planning/ai-workstreams.json').read_text(encoding='utf-8'))
plan = json.loads((root/board['status_source']).read_text(encoding='utf-8'))
sessions = {s['id']: s for s in plan['sessions']}
queue = json.loads((root/board['task_source']).read_text(encoding='utf-8')) if board.get('task_source') else {'tasks': []}
tasks = {}
for task in queue['tasks']:
    tid = task['id']
    if tid in tasks: sys.exit('Duplicate task: '+tid)
    if task['owner'] not in ('Codex', 'Claude', 'Human'): sys.exit('Unknown task owner: '+tid)
    if task['session'] not in sessions: sys.exit('Unknown task parent: '+tid)
    if task['state'] not in ('queued', 'claimed', 'in_progress', 'blocked', 'ready_for_review', 'complete'):
        sys.exit('Unknown task state: '+tid)
    if type(task['priority']) is not int or task['priority'] < 1: sys.exit('Invalid task priority: '+tid)
    if not (root/task['handoff']).is_file(): sys.exit('Missing task handoff: '+task['handoff'])
    tasks[tid] = task
for tid, task in tasks.items():
    if any(d not in tasks for d in task['depends_on']): sys.exit('Unknown task dependency: '+tid)
visited, visiting = set(), set()
def visit(tid):
    if tid in visiting: sys.exit('Task dependency cycle: '+tid)
    if tid in visited: return
    visiting.add(tid)
    for dep in tasks[tid]['depends_on']: visit(dep)
    visiting.remove(tid)
    visited.add(tid)
for tid in tasks: visit(tid)
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
    print(f'PASS: {len(sessions)} canonical markers, each assigned once; handoffs and dependencies exist. No status is duplicated. {len(tasks)} bounded tasks validated separately.')
    sys.exit(0)
if args.session and args.session not in sessions:
    sys.exit('Unknown session: '+args.session)
if args.task and args.task not in tasks: sys.exit('Unknown task: '+args.task)
if args.task and args.owner and tasks[args.task]['owner'] != args.owner: sys.exit('Task does not belong to requested owner: '+args.task)
if args.task and args.session and tasks[args.task]['session'] != args.session: sys.exit('Task does not belong to requested session: '+args.task)
print('Integration branch: '+board['integration_branch'])
if args.tasks or args.task:
    print('Bounded task states do not change canonical session status.')
    for task in sorted(tasks.values(), key=lambda t:(t['priority'], t['owner'], t['id'])):
        if args.task and task['id'] != args.task: continue
        if args.owner and task['owner'] != args.owner: continue
        if args.session and task['session'] != args.session: continue
        if not args.task and task['state'] == 'complete': continue
        unmet = [d for d in task['depends_on'] if tasks[d]['state'] != 'complete']
        print(f"\n{task['id']} | {task['owner']} | {task['state']} | priority {task['priority']} | {task['title']}")
        print('  Parent: '+task['session']+' ('+sessions[task['session']]['status']+')')
        print('  Waiting for: '+(', '.join(unmet) if unmet else 'no incomplete task dependency; follow handoff scope'))
        if task.get('branch'): print('  Existing branch: '+task['branch'])
        print('  Next: '+task['next'])
        print('  Handoff: '+task['handoff'])
    sys.exit(0)
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
